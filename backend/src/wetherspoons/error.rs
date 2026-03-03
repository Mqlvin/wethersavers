use thiserror::Error;


#[derive(Error, Debug)]
pub enum WetherspoonsError {
    #[error("Couldn't get Wetherspoons venues: {0}")]
    GetVenuesError(String),

    #[error("Couldn't parse Wetherspoons venue JSON: {0}")]
    ParseVenuesError(String),

    #[error("Couldn't parse Wetherspoons sales ID data: {0}")]
    ParseSalesIdError(String),

    #[error("Couldn't parse Wetherspoons menu listing data: {0}")]
    ParseMenusError(String),

    #[error("Couldn't parse Wetherspoons drink menu data: {0}")]
    ParseDrinksError(String),
}
