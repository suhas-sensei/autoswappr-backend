use axum::{
    routing::{get, post},
    Router,
};
mod health_check;
mod types;
mod unsubscription;
mod subscription;

use crate::AppState;

// Application router.
// All routes should be merged here.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route("/unsubscribe", post(unsubscription::handle_unsubscribe))
        .route("/subscriptions", post(subscription::create_subscription))
}
