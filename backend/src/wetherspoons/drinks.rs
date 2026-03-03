use axum::http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::wetherspoons::{API_AUTH, API_ENDPOINT, error::WetherspoonsError};


#[derive(Serialize, Deserialize, Clone)]
pub struct DrinksMenu {
    data: Value // generic value
}


pub struct Drink {
    name: String,
    category: String,
    strength: f32,
    portions: Vec<Portion>,
}

pub struct Portion {
    amount: usize, // ml
    price: usize, // 120 eg is £1.20
    ppu: f32 // pennies per unit alc
}



pub async fn get_drinks_menu(venue_identifier: usize, sales_id: usize, drinks_menu_id: usize) -> Result<usize, WetherspoonsError> {
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

    let categories: Vec<Value> = match menu.data.get("categories") {
        Some(cat) => { 
            match cat.as_array() {
                Some(arr) => arr.to_vec(),
                None => { return Err(WetherspoonsError::ParseDrinksError("Couldn't parse drinks categories to array".to_string())); }
            }
        },
        None => { return Err(WetherspoonsError::ParseDrinksError("Couldn't find drinks categories".to_string())); }
    };



    for category in categories {
        let name = category.get("name").unwrap_or(&Value::String("unknown".to_string()));
        
        if let None = category.get("itemGroups") {
            return Err(WetherspoonsError::ParseDrinksError("Couldn't find itemGroups in drinks menu".to_string()));
        }

        for item in category.get("itemGroups").expect("The error was caught previously") {
            // parse each item
        }
    }

    

    Ok(0)
}
