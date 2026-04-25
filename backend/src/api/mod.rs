use std::{io::Write, sync::Arc};

use axum::{Json, Router, extract::Path, routing::get};
use flate2::{Compression, write::GzEncoder};
use serde::Serialize;

use crate::wetherspoons::{cache::{get_cached_drinks_menu, get_cached_venues_encoded, has_valid_drinks_cache}, drinks::{Drink, get_drinks_menu}, get_drinks_menu_id, get_sales_int, get_venues};

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
            match get_cached_venues_encoded().await {
                Ok(str) => {
                    return Json(Response::ok((&*str).clone()));
                },
                Err(err) => { return Json(Response::err(err.to_string())) }
            }
        }))
        .route("/drinks/{id}", get(|Path(id): Path<usize>| async move {
            let mut arc_drinks = if !has_valid_drinks_cache(&id) {
                let sales_id = match get_sales_int(&id).await {
                    Ok(val) => val,
                    Err(err) => { return Json(Response::err(err.to_string())); }
                };

                let drinks_menu_id = match get_drinks_menu_id(&id, &sales_id).await {
                    Ok(val) => val,
                    Err(err) => { return Json(Response::err(err.to_string())); }
                };

                match get_cached_drinks_menu(&id, &sales_id, &drinks_menu_id).await {
                    Ok(val) => val,
                    Err(err) => { return Json(Response::err(err.to_string())); }
                }
            } else {
                match get_cached_drinks_menu(&id, &0, &0).await {
                    Ok(val) => val,
                    Err(err) => { return Json(Response::err(err.to_string())); }
                }
            };


            let drinks: &mut Vec<Drink> = Arc::make_mut(&mut arc_drinks);
            drinks.sort_by(|a, b| a.portions.first().expect("At least one portion").ppu.total_cmp(&b.portions.first().expect("At least one portion").ppu));

            let json_string = match serde_json::to_string(&drinks) {
                Ok(str) => str,
                Err(err) => { return Json(Response::err(format!("Failed to drinks JSON to string: {}", err.to_string()))) }
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

            Json(Response::ok(base64))
        }))
}
