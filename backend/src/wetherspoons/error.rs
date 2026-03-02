use thiserror::Error;


#[derive(Error, Debug)]
pub enum WetherspoonsError {
    #[error("Couldn't get Wetherspoons venues: {0}")]
    GetVenuesError(String),

    #[error("Couldn't parse Wetherspoons venue JSON: {0}")]
    ParseVenuesError(String),
}
