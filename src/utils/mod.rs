use tower_http::cors::{Any, CorsLayer}; // import CORS layer

// CORS util fn
pub fn cors_handler() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}