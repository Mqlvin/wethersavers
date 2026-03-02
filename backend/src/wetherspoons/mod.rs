use std::io::Read;

use flate2::bufread::GzDecoder;
use serde::{Deserialize, Serialize};
use crate::wetherspoons::error::WetherspoonsError;

mod error;

const VENUE_ENDPOINT: &str = "https://oandp-appmgr-prod.s3.eu-west-2.amazonaws.com/global.json";
const API_ENDPOINT: &str = "https://ca.jdw-apps.net/api/v0.1";
const API_AUTH: &str = "Bearer 1|SFS9MMnn5deflq0BMcUTSijwSMBB4mc7NSG2rOhqb2765466";


#[derive(Serialize, Deserialize, Clone)]
pub struct Venue {
    pub identifier: Option<u64>,
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
