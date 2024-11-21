use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::{post, get}, Json, Router};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use crate::utils;

async fn say_gm() -> &'static str {
    "GM"
}

pub fn routes() -> Router {
    // add CORS layer to allow any origin
    // let cors = utils::cors_handler();

    // integrate CORS to router
    Router::new()
        .route("/", get(say_gm))
        // .layer(cors)
}