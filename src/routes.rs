use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, Rust Web App!"
}

async fn health() -> &'static str {
    "OK"
}

async fn city_info() -> &'static str {
    "Indore: Sample route for city info"
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/city", get(city_info))
}
