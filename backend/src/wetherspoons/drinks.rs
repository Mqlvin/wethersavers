use std::collections::{HashMap, HashSet};

use axum::http::{HeaderMap, HeaderValue};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

use crate::wetherspoons::{API_AUTH, API_ENDPOINT, error::WetherspoonsError};


static VOLUMES: Lazy<HashMap<&'static str, u32>> = Lazy::new(|| {
    HashMap::from([
        ("Pint", 568),
        ("Half pint", 284),
        ("Half Pint", 284),
        ("Half", 284),
        ("Single", 25),
        ("Double", 50)
    ])
});

static DRINK_CATEGORIES: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "Lager, beer, stout and craft | Draught",
        "Cider | Draught and bottles ",
        "Real ale",
        "Craft | Draught, bottles & cans",
        "World Beers | Bottles",
        "Premixed drinks",
        "Wine, prosecco & sparkling",
        "Spritz cocktails",
        "Cocktails and BuzzBallz",
        "Vodka",
        "Gin",
        "Rum",
        "Whiskey",
        "Tequila",
        "Liqueurs, cognac and brandy ",
        "Bombs and shots",
        "2 for £6.50",
        "3 for £5.10",
        "4 for £5",
    ]
});

static PARSE_QTY_PRICE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^\s*(\d+)\s*for\s*£\s*([0-9]+(?:\.[0-9]+)?)\s*$").unwrap()
});


#[derive(Serialize, Deserialize, Clone)]
pub struct DrinksMenu {
    data: Value // generic value
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Drink {
    pub name: String,
    pub category: String,
    pub medium: String,
    pub strength: f32,
    pub portions: Vec<Portion>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Portion {
    pub amount: u32, // ml
    pub price: f32, // 120 eg is £1.20
    pub strength: f32, // abv
    pub ppu: f32 // price (in pence) per unit
}



pub async fn get_drinks_menu(venue_identifier: &usize, sales_id: &usize, drinks_menu_id: &usize) -> Result<Vec<Drink>, WetherspoonsError> {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(API_AUTH).expect("Could insert the auth header"));
    headers.insert("User-Agent", HeaderValue::from_str("Wetherspoons App").expect("Could insert the auth header"));

    let menus_json = match reqwest::Client::new().get(format!("{}/jdw/venues/{}/sales-areas/{}/menus/{}", API_ENDPOINT, venue_identifier, sales_id, drinks_menu_id)).headers(headers).send().await {
        Ok(resp) => {
            match resp.text().await {
                Ok(json_text) => json_text,
                Err(err) => {
                    return Err(WetherspoonsError::ParseMenusError(format!("Couldn't get response text: {}", err.to_string()).to_string()));
                }
            }

        },
        Err(err) => {
            return Err(WetherspoonsError::ParseMenusError(format!("Couldn't request the detailed venue endpoint: {}", err.to_string()).to_string()));
        }
    };

    let menu: DrinksMenu = match serde_json::from_str(&menus_json) {
        Ok(obj) => obj,
        Err(err) => {
            return Err(WetherspoonsError::ParseMenusError(err.to_string()));
        }
    };

    // this will be equal to the array of category objs, or return an err
    let categories: Vec<Value> = match menu.data.get("categories") {
        Some(cat) => { 
            match cat.as_array() {
                Some(arr) => arr.to_vec(),
                None => { return Err(WetherspoonsError::ParseDrinksError("Couldn't parse drinks categories to array".to_string())); }
            }
        },
        None => { return Err(WetherspoonsError::ParseDrinksError("Couldn't find drinks categories".to_string())); }
    };

    
    let mut drink_accumulator: Vec<Drink> = vec![];
    let multibuy_options: Vec<(String, u8, f32)> = extract_multibuy_categories(&categories);
    let multibuy_category_names: Vec<&str> = multibuy_options.iter().map(|o| o.0.as_str()).collect();

    for category in categories {
        let name_fallback = Value::String("Unknown".to_string());
        let name = category.get("name").unwrap_or(&name_fallback).as_str().unwrap_or("Unknown");
        if !DRINK_CATEGORIES.contains(&name) && !multibuy_category_names.contains(&name) { continue; } // ignore drinks outside alcoholic categories

        if let None = category.get("itemGroups") {
            return Err(WetherspoonsError::ParseDrinksError("Couldn't find itemGroups in drinks menu".to_string()));
        }

        // This iterates sub-categories, such as Lager, Craft, Ale, etc in the biggest "Draught" category for example
        for item in category.get("itemGroups").expect("The error was caught previously").as_array().expect("itemGroups wasn't an array") {
            let sub_category_name = match item.get("name") {
                Some(value) => {
                    if value.is_null() {
                        name // fallback to category name
                    } else {
                        value.as_str().unwrap_or(name) // fallback to category name, if required
                    }
                }
                None => { name } // fallback to category name
            }.to_string();

            // if the item category is a multi-buy deal, override the items price price before
            // adding to the app
            let price_override = if let Some((_, qty, price)) = multibuy_options.iter().find(|c| c.0 == name) {
                Some(price / *qty as f32)
            } else {
                None
            };
            drink_accumulator.append(&mut parse_item_list(item.get("items").unwrap().as_array().unwrap().to_vec(), sub_category_name, price_override));
        }
    }

    post_process_drinks(&mut drink_accumulator);
    Ok(drink_accumulator)
}



// price override is used in the case of multibuy options such as 2 for £5
// since all items in that case are 2.50, we can override all items below with the associated price
fn parse_item_list(items: Vec<Value>, sub_category_name: String, price_override: Option<f32>) -> Vec<Drink> {
    let mut drinks: Vec<Drink> = vec![];

    for item in items {
        if !is_product(&item) { continue; }
        if is_out_of_stock(&item) { continue; }

        // get name of item
        let name = match get_item_name(&item) {
            Some(name) => name,
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Couldn't find item name, ignoring.");
                continue;
            }
        };

        // get strength of item, or return (probs not alcoholic)
        let strength: f32 = match extract_abv(&item) {
            Some(val) => val,
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Couldn't find item ABV for item '{}', ignoring.", name);
                continue;
            }
        };

        // get description as array
        let description = match get_global_description(&item) {
            Some(desc) => desc,
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Couldn't find portions object, ignoring.");
                continue;
            }
        };

        // get portions obj as array
        let portion_obj: Vec<Value> = match get_portions_array(&item) {
            Some(portions) => portions,
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Couldn't find portions object, ignoring.");
                continue;
            }
        };

        // otherwise parse the portions (unknown type) json object into typed struct, apply price
        // override if required
        let (mut portions, medium) = match parse_portions_obj(&portion_obj, strength, description, price_override) {
            Some(portions) => portions,
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Couldn't parse portions object, ignoring.");
                continue;
            }
        };

        // if we have multiple portions and a price override, we have a problem
        if price_override.is_some() && portions.len() != 1 {
            #[cfg(debug_assertions)]
            // eprintln!("Item '{}' was part of a deal (price override active) however there were {} portions", name, portions.len());
            continue;
        }

        portions.sort_by(|a, b| a.ppu.total_cmp(&b.ppu));
        drinks.push(Drink { name: if price_override.is_some() { format!("[{}] {}", sub_category_name, name).to_string() } else { name.to_string() }, category: sub_category_name.clone(), medium, strength, portions });
    }

    drinks
}

fn is_product(item: &Value) -> bool {
    if let Some(item_type) = item.get("itemType") {
        if item_type.as_str().unwrap_or("notproduct") != "product" {
            return false; // itemType is not a product
        }
    } else {
        return false; // continue if itemType not present
    }
    return true;
}

fn is_out_of_stock(item: &Value) -> bool {
    if let Some(out_of_stock) = item.get("isOutOfStock") {
        if out_of_stock.as_bool().unwrap_or(true) {
            return true; // product out of stock
        }
    } else {
        return true; // stock unknown
    }
    return false;
}

fn get_item_name(item: &Value) -> Option<String> {
    match item.get("name") {
        Some(name) => {
            match name.as_str() {
                Some(val) => Some(val.to_string()),
                None =>  None
            }
        },
        None => None
    }
}

fn get_global_description(item: &Value) -> Option<String> {
    match item.get("description") {
        Some(name) => {
            match name.as_str() {
                Some(val) => Some(val.to_string()),
                None =>  None
            }
        },
        None => None
    }
}

fn extract_abv(item: &Value) -> Option<f32> {
    match item.get("description") {
        Some(desc) => {
            match desc.as_str() {
                Some(desc_str) => {
                    let safe_str = if desc_str.is_empty() {
                        return None
                    } else {
                        desc_str
                    };
                    match extract_abv_from_desc(safe_str) {
                        Some(abv) => match abv.parse::<f32>() {
                            Ok(res) => Some(res),
                            Err(_) => None
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        None => None
    }
}

fn extract_abv_from_desc(desc: &str) -> Option<String> {
    // Find "% ABV" or "%ABV"
    let percent_pos = match desc.find('%') {
        Some(idx) => idx,
        None => { return None; }
    };
    let after_percent = &desc[percent_pos + 1..];

    // Must be followed by optional space then "ABV"
    let rest = after_percent.trim_start();
    if !rest.starts_with("ABV") {
        return None;
    }

    // Walk backwards from '%' to collect the number (and dot) chars
    let before = &desc[..percent_pos];
    let mut start = before.len();

    for (i, ch) in before.char_indices().rev() {
        if ch.is_ascii_digit() || ch == '.' {
            start = i;
        } else {
            break;
        }
    }

    if start == before.len() {
        return None;
    }

    Some(before[start..percent_pos].to_string())
}

fn get_portions_array(item: &Value) -> Option<Vec<Value>> {
    match item.get("options") {
        Some(obj) => {
            match obj.get("portion") {
                Some(portion_obj) => {
                    match portion_obj.get("options") {
                        Some(portion_options) => {
                            if let Some(ok_portion_options) = portion_options.as_array() {

                                if ok_portion_options.is_empty() { // dont return it if its empty / null
                                    None
                                } else {
                                    Some(ok_portion_options.to_vec())
                                }

                            } else {
                                None
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        },
        None => None
    }
}

// returns portions, can/pint/etc
fn parse_portions_obj(portion_obj: &Vec<Value>, item_strength: f32, global_description: String, price_override: Option<f32>) -> Option<(Vec<Portion>, String)> {
    // array to put Rust-struct portions in
    let mut portions: Vec<Portion> = Vec::with_capacity(4);
    let mut drink_medium: Option<String> = None;

    for portion in portion_obj {
        if price_override.is_some() {
            if let Some(is_default_val) = portion.get("isDefault") {
                if let Some(is_default) = is_default_val.as_bool() {
                    if !is_default {
                        continue;
                    }
                    // it is default here, in which case just move on
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        let value_obj = match portion.get("value") { 
            Some(val) => val,
            None => { continue; }
        };

        let price = if let Some(overridden_price) = price_override {
            overridden_price
        } else {
            match value_obj.get("price") {
                Some(price_obj) => {
                    match price_obj.get("value") {
                        Some(value) => match value.as_f64() {
                            Some(price) => {
                                price as f32
                            },
                            None => { continue; }
                        },
                        None => { continue; }
                    }
                },
                None => { continue; }
            }
        };


        let volume_extract: Option<u32> = match value_obj.get("name") {
            Some(volume_name) => {
                let volume_name = match volume_name.as_str() {
                    Some(val) => val,
                    None => { continue; }
                };

                drink_medium = Some(volume_name.to_string());

                // handling for specific volumes
                if volume_name.to_lowercase() == "bottle" {
                    match value_obj.get("description") {
                        Some(desc) => {
                            match desc.as_str() {
                                Some(val) => {
                                    match extract_volume_from_desc(val) {
                                        Some(vol) => { Some(vol) },
                                        None => { None }
                                    }
                                },
                                None => { None }
                            }
                        },
                        None => { None }
                    }
                } else if volume_name.to_lowercase() == "can" {
                    let mut desc_to_use = match value_obj.get("description") {
                        Some(desc) => {
                            match desc.as_str() {
                                Some(val) => {
                                    val
                                },
                                None => { &global_description }
                            }
                        },
                        None => { &global_description }
                    };
                    if desc_to_use.is_empty() {
                        desc_to_use = &global_description;
                    }
                    match extract_volume_from_desc(desc_to_use) {
                        Some(vol) => { Some(vol) },
                        None => { None }
                    }
                } else {
                    match VOLUMES.get(volume_name) {
                        Some(vol) => Some(*vol),
                        None => { None }   
                    }
                }
            },
            None => { 
                match extract_volume_from_desc(&global_description) {
                    Some(vol) => { Some(vol) },
                    None => {
                        #[cfg(debug_assertions)]
                        eprintln!("Couldn't extract vol from fallback: {}", global_description);
                        None
                    }
                }
            }
        };

        if let None = volume_extract {
            continue;
        }

        portions.push(
            Portion {
                amount: volume_extract.unwrap(),
                price: price,
                strength: item_strength,
                ppu: (price) / ((volume_extract.unwrap() as f32 * (item_strength / 100.)) / 10.)
            }
        );
    }

    Some((portions, drink_medium.unwrap_or("".to_string())))
}

fn extract_volume_from_desc(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Skip non-digits
        if !chars[i].is_ascii_digit() {
            i += 1;
            continue;
        }

        // Collect a run of digits
        let start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        let end = i;

        // Optional whitespace after the number
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        // Check for "ml" after the digits (and optional spaces)
        if i + 1 < chars.len()
            && (chars[i] == 'm' || chars[i] == 'M')
            && (chars[i + 1] == 'l' || chars[i + 1] == 'L')
        {
            let num_str: String = chars[start..end].iter().collect();
            if let Ok(value) = num_str.parse::<u32>() {
                return Some(value);
            }
        }
        // Otherwise continue scanning
    }

    None
}

fn post_process_drinks(drinks: &mut Vec<Drink>) {
    drinks.retain(|drink| drink.portions.len() > 0 && drink.strength >= 0.05);

    let mut seen_drinks: HashSet<(String, String)> = HashSet::new();
    drinks.retain(|drink| {
        seen_drinks.insert((drink.name.clone(), drink.medium.clone()))
    });

    drinks.iter_mut().for_each(|drink| {
        if let Some(first_ppu) = drink.portions.first().map(|p| p.ppu) {
            drink.portions.retain(|p| p.ppu == first_ppu);
        }

        drink.portions.sort_by(|a, b| b.amount.cmp(&a.amount));

        if drink.portions.len() > 1 {
            drink.portions.truncate(1);
        }
    });
}

// Return a list of categories names, and their amount and price, etc [("2 for £5", 2, 5.00), ...]
fn extract_multibuy_categories(categories: &Vec<Value>) -> Vec<(String, u8, f32)> {
    let mut multibuy_categories: Vec<(String, u8, f32)> = Vec::new();

    for category in categories {
        let name = match category.get("name") {
            Some(val) => {
                match val.as_str() {
                    Some(name) => name,
                    None => continue
                }
            }
            None => continue
        };
        
        if let Some((qty, price)) = parse_qty_price(&name) {
            multibuy_categories.push((name.to_string(), qty, price));
        }
    }

    multibuy_categories
}

fn parse_qty_price(name: &str) -> Option<(u8, f32)> {
    PARSE_QTY_PRICE.captures(name).and_then(|cap| {
        let qty = cap.get(1)?.as_str().parse::<u8>().ok()?;
        let price = cap.get(2)?.as_str().parse::<f32>().ok()?;
        Some((qty, price))
    })
}
