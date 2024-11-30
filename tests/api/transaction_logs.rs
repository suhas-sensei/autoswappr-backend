use crate::helpers::TestApp;
use autoswappr_backend::service::transaction_logs::log_transaction;
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::json;

#[tokio::test]
async fn test_transaction_log_service_with_valid_payload() {
    let address = "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF";
    let from_token = "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF";
    let to_token = "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF";
    let percentage = 50;
    let amount_from = 4000;
    let amount_to = 2000;

    let app = TestApp::new().await;
    let tx = log_transaction(
        address,
        from_token,
        to_token,
        percentage,
        amount_from,
        amount_to,
        &app.db.pool,
    )
    .await
    .unwrap();
    assert_eq!(tx.wallet_address, address);
    assert_eq!(tx.from_token, from_token);
    assert_eq!(tx.to_token, to_token);
    assert_eq!(tx.percentage, percentage);
    assert_eq!(tx.amount_from, amount_from);
    assert_eq!(tx.amount_to, amount_to);
}

#[tokio::test]
async fn test_transaction_log_service_with_invalid_address() {
    let address = "0xF1d2eD1a7d9A2aE3c467Bc72C5dF";
    let from_token = "0xF1d2eD1a7d9A2aE3c467Bc2Cojojoj5dF";
    let to_token = "0xF1d2eD1a7d9A2aE3c467Bc72C5iohhosdF";
    let percentage = 50;
    let amount_from = 4000;
    let amount_to = 2000;

    let app = TestApp::new().await;
    let result = log_transaction(
        address,
        from_token,
        to_token,
        percentage,
        amount_from,
        amount_to,
        &app.db.pool,
    )
    .await;

    assert!(result.is_err())
}

#[tokio::test]
async fn test_transaction_log_request_with_valid_payload() {
    let app = TestApp::new().await;
    let response = app
        .request(
            Request::builder()
                .method("POST")
                .uri("/log_transaction")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "wallet_address": "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF",
                        "from_token": "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF",
                        "to_token": "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF",
                        "percentage": 50,
                        "amount_from": 5000,
                        "amount_to": 4000
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 16).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "success");
    assert_eq!(json["message"], "transaction logged successfully");
}

#[tokio::test]
async fn test_transaction_log_request_with_invalid_payload() {
    let app = TestApp::new().await;
    let response = app
        .request(
            Request::builder()
                .method("POST")
                .uri("/log_transaction")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "wallet_address": "0xF1d2eD1a7d9A2aE3c467Bc726946e2C5dF", // invalid data
                        "from_token": "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF",
                        "to_token": "0xF1d2eD1a7d9A2aE3c467Bc72694C9B8C16e2C5dF",
                        "percentage": 500, // invalid data
                        "amount_from": 5000,
                        "amount_to": 4000
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
