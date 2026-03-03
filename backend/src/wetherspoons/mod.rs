use std::io::Read;

use axum::http::{HeaderMap, HeaderValue};
use flate2::bufread::GzDecoder;
use serde::{Deserialize, Serialize};
use crate::wetherspoons::error::WetherspoonsError;

mod error;
mod drinks;

const VENUE_ENDPOINT: &str = "https://oandp-appmgr-prod.s3.eu-west-2.amazonaws.com/global.json";
const API_ENDPOINT: &str = "https://ca.jdw-apps.net/api/v0.1";
const API_AUTH: &str = "Bearer 1|SFS9MMnn5deflq0BMcUTSijwSMBB4mc7NSG2rOhqb2765466";


/*
* Venue Related Queries
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct Venue {
    pub identifier: Option<usize>,
    pub name: Option<String>,
    pub town: Option<String>,
    pub county: Option<String>,
    pub post_code: Option<String>,
    pub is_closed: Option<u8>,
}

#[derive(Deserialize)]
struct VenuesData {
    venues: Vec<Venue>
}

pub async fn get_venues() -> Result<Vec<Venue>, WetherspoonsError> {
    let gzipped = match reqwest::get(VENUE_ENDPOINT).await {
        Ok(resp) => {

            match resp.bytes().await {
                Ok(gzipped_bytes) => gzipped_bytes,
                Err(err) => {
                    return Err(WetherspoonsError::GetVenuesError(format!("Couldn't get response text: {}", err.to_string()).to_string()));
                }
            }

        },
        Err(err) => {
            return Err(WetherspoonsError::GetVenuesError(format!("Couldn't request the venue endpoint: {}", err.to_string()).to_string()));
        }
    };

    let mut decoder = GzDecoder::new(&*gzipped);
    let mut decompressed_body = String::new();
    if let Err(err) = decoder.read_to_string(&mut decompressed_body) {
        return Err(WetherspoonsError::ParseVenuesError(err.to_string()));
    };

    let venues_data: VenuesData = match serde_json::from_str(decompressed_body.as_str()) {
        Ok(venues) => venues,
        Err(err) => {
            return Err(WetherspoonsError::ParseVenuesError(err.to_string()));
        }
    };

    return Ok(venues_data.venues.iter().filter(|venue| venue.is_closed.unwrap_or(1u8) == 0u8).cloned().collect());
}


/*
* Sales Area Related Queries
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct VenueDetails {
    data: VenueBarNumber
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VenueBarNumber {
    #[serde(rename = "salesAreas")]
    singleton: Vec<VenueSalesArea>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VenueSalesArea {
    id: usize
}

pub async fn get_sales_int(venue_identifier: usize) -> Result<usize, WetherspoonsError> {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(API_AUTH).expect("Could insert the auth header"));
    headers.insert("User-Agent", HeaderValue::from_str("Wetherspoons App").expect("Could insert the auth header"));

    let gzipped = match reqwest::Client::new().get(format!("{}/venues/{}", API_ENDPOINT, venue_identifier)).headers(headers).send().await {
        Ok(resp) => {
            match resp.text().await {
                Ok(gzipped_bytes) => gzipped_bytes,
                Err(err) => {
                    return Err(WetherspoonsError::ParseSalesIdError(format!("Couldn't get response text: {}", err.to_string()).to_string()));
                }
            }
        },
        Err(err) => {
            return Err(WetherspoonsError::ParseSalesIdError(format!("Couldn't request the detailed venue endpoint: {}", err.to_string()).to_string()));
        }
    };

    let venue_details: VenueDetails = match serde_json::from_str(gzipped.as_str()) {
        Ok(obj) => obj,
        Err(err) => {
            return Err(WetherspoonsError::ParseSalesIdError(err.to_string()));
        }
    };

    match venue_details.data.singleton.first() {
        Some(area) => Ok(area.id),
        None => Err(WetherspoonsError::ParseSalesIdError("Couldn't find sales bar ID in venue details".to_string()))
    }
}

/*
* Drink Menu Getter Queries
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct MenuListing {
    data: Vec<Menu>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Menu {
    id: usize,
    name: String
}

pub async fn get_drinks_menu_id(venue_identifier: usize, sales_id: usize) -> Result<usize, WetherspoonsError> {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(API_AUTH).expect("Could insert the auth header"));
    headers.insert("User-Agent", HeaderValue::from_str("Wetherspoons App").expect("Could insert the auth header"));

    let menus_json = match reqwest::Client::new().get(format!("{}/jdw/venues/{}/sales-areas/{}/menus", API_ENDPOINT, venue_identifier, sales_id)).headers(headers).send().await {
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

    let menus: MenuListing = match serde_json::from_str(&menus_json) {
        Ok(obj) => obj,
        Err(err) => {
            return Err(WetherspoonsError::ParseMenusError(err.to_string()));
        }
    };

    for menu in menus.data {
        if menu.name.to_ascii_lowercase() == "drinks" {
            return Ok(menu.id);
        }
    }

    return Err(WetherspoonsError::ParseMenusError("No menu found with name 'drinks'".to_string()));
}


