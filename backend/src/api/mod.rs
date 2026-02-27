use axum::{Router, routing::get};


pub fn get_api_router() -> Router {
    Router::new()
        .route("/test", get(|| async { tokio::time::sleep(std::time::Duration::from_secs(2)).await; "data after 2s" }))
}
