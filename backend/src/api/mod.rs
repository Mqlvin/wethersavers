use std::io::Write;

use axum::{Json, Router, routing::get};
use flate2::{Compression, write::GzEncoder};
use serde::Serialize;

use crate::wetherspoons::{get_drinks_menu_id, get_sales_int, get_venues};

#[derive(Serialize)]
pub struct Response<T>
    where T: Serialize
{
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            success: true,
            error_reason: None,
            data: Some(data)
        }
    }

    pub fn err(error_reason: impl Into<String>) -> Self {
        Response {
            success: false,
            error_reason: Some(error_reason.into()),
            data: None
        }
    }
}



pub fn get_api_router() -> Router {
    Router::new()
        .route("/venues", get(|| async {
            match get_venues().await {
                Ok(venues) => {
                    let json_string = match serde_json::to_string(&venues) {
                        Ok(str) => str,
                        Err(err) => { return Json(Response::err(format!("Failed to convert venue JSON to string: {}", err.to_string()))) }
                    };

                    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                    if let Err(err) = encoder.write_all(&json_string.as_bytes()) {
                        return Json(Response::err(format!("Failed to write bytes to GZIP compressor: {}", err.to_string())));
                    };
                    let result = match encoder.finish() {
                        Ok(res) => res,
                        Err(err) => { return Json(Response::err(format!("Failed to write bytes to GZIP compressor: {}", err.to_string()))); } 
                    };

                    let base64 = base64::encode(&result);

                    return Json(Response::ok(base64));
                },
                Err(err) => { return Json(Response::err(err.to_string())) }
            }
        }))
        .route("/getsalesid", get(|| async {
            let test = get_sales_int(5600).await.expect("got error id");
            println!("{}", get_drinks_menu_id(5600, test).await.unwrap());
            Json(Response::ok(get_sales_int(5600).await.unwrap_or(usize::MAX)))
        }))
}
