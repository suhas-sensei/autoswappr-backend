use axum::{routing::get, Router};
mod health_check;
mod types;

use crate::AppState;

// Application router.
// All routes should be merged here.
pub fn router() -> Router<AppState> {
    Router::new().route("/health_check", get(health_check::health_check))
}
