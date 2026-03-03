mod frontend;
mod api;
mod wetherspoons;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::api::get_api_router;

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .fallback(frontend::static_handler)
        .nest("/api", get_api_router())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
