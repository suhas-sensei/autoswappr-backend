use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;

use crate::helpers::*;

#[tokio::test]
async fn test_subscribe_ok() {
    let app = TestApp::new().await;

    let payload = json!({
        "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        "to_token": "0x1234567890123456789012345678901234567890",
        "from_token": [
            "0xabcdef0123456789abcdef0123456789abcdef01",
            "0x9876543210987654321098765432109876543210"
        ],
        "percentage": [60, 40]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_successful_subscription_creation() {
    let app = TestApp::new().await;

    sqlx::query!("DELETE FROM swap_subscription_from_token")
        .execute(&app.db.pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM swap_subscription")
        .execute(&app.db.pool)
        .await
        .unwrap();

    let wallet_address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
    let to_token = "0x1234567890123456789012345678901234567890";
    let from_tokens = vec![
        "0xabcdef0123456789abcdef0123456789abcdef01",
        "0x9876543210987654321098765432109876543210",
    ];
    let percentages = vec![60, 40];

    let payload = json!({
        "wallet_address": wallet_address,
        "to_token": to_token,
        "from_token": from_tokens,
        "percentage": percentages
    });

    let req = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let subscription = sqlx::query!(
        r#"
        SELECT wallet_address, to_token, is_active 
        FROM swap_subscription 
        WHERE wallet_address = $1
        "#,
        wallet_address
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    assert_eq!(subscription.wallet_address, wallet_address);
    assert_eq!(subscription.to_token, to_token);
    assert!(subscription.is_active);

    let from_token_records = sqlx::query!(
        r#"
        SELECT from_token, percentage
        FROM swap_subscription_from_token
        WHERE wallet_address = $1
        "#,
        wallet_address
    )
    .fetch_all(&app.db.pool)
    .await
    .unwrap();

    assert_eq!(from_token_records.len(), 2);

    assert_eq!(from_token_records[0].from_token, from_tokens[1]);
    assert_eq!(from_token_records[0].percentage as i32, percentages[1]);

    assert_eq!(from_token_records[1].from_token, from_tokens[0]);
    assert_eq!(from_token_records[1].percentage as i32, percentages[0]);
}

#[tokio::test]
async fn test_invalid_subscription_percentage() {
    let app = TestApp::new().await;

    sqlx::query!("DELETE FROM swap_subscription_from_token")
        .execute(&app.db.pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM swap_subscription")
        .execute(&app.db.pool)
        .await
        .unwrap();

    let payload = json!({
        "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        "to_token": "0x1234567890123456789012345678901234567890",
        "from_token": [
            "0xabcdef0123456789abcdef0123456789abcdef01",
            "0x9876543210987654321098765432109876543210"
        ],
        "percentage": [20, 40]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_percentage_length() {
    let app = TestApp::new().await;

    sqlx::query!("DELETE FROM swap_subscription_from_token")
        .execute(&app.db.pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM swap_subscription")
        .execute(&app.db.pool)
        .await
        .unwrap();

    let payload = json!({
        "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        "to_token": "0x1234567890123456789012345678901234567890",
        "from_token": [
            "0xabcdef0123456789abcdef0123456789abcdef01",
            "0x9876543210987654321098765432109876543210"
        ],
        "percentage": [20]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_wallet_address() {
    let app = TestApp::new().await;

    sqlx::query!("DELETE FROM swap_subscription_from_token")
        .execute(&app.db.pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM swap_subscription")
        .execute(&app.db.pool)
        .await
        .unwrap();

    let payload = json!({
        "wallet_address": "invalid_wallet_address",
        "to_token": "0x1234567890123456789012345678901234567890",
        "from_token": [
            "0xabcdef0123456789abcdef0123456789abcdef01",
            "0x9876543210987654321098765432109876543210"
        ],
        "percentage": [20, 80]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_to_token_address() {
    let app = TestApp::new().await;

    sqlx::query!("DELETE FROM swap_subscription_from_token")
        .execute(&app.db.pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM swap_subscription")
        .execute(&app.db.pool)
        .await
        .unwrap();

    let payload = json!({
        "wallet_address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        "to_token": "invalid_to_token",
        "from_token": [
            "0xabcdef0123456789abcdef0123456789abcdef01",
            "0x9876543210987654321098765432109876543210"
        ],
        "percentage": [20, 80]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
