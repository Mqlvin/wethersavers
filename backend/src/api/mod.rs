use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
    where T: Serialize
{
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            success: true,
            error_reason: None,
            data: Some(data)
        }
    }

    pub fn err(error_reason: impl Into<String>) -> Self {
        ApiResponse {
            success: false,
            error_reason: Some(error_reason.into()),
            data: None
        }
    }
}


#[derive(Serialize)]
struct ThreeSecondResponse {
    text: String,
    num: u64,
}


pub fn get_api_router() -> Router {
    Router::new()
        .route("/test", get(|| async { tokio::time::sleep(std::time::Duration::from_secs(2)).await; Json(ApiResponse::ok(ThreeSecondResponse { text: "Hi".to_string(), num: 3 })) }))
        .route("/testfail", get(|| async { tokio::time::sleep(std::time::Duration::from_secs(2)).await; Json(ApiResponse::<()>::err("this is err reason")) }))
}
