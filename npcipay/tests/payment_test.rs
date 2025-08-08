
use httpmock::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize)]
struct UpiPaymentRequest {
    payer_upi: String,
    payee_upi: String,
    amount: f64,
    txn_note: String,
    merchant_id: String,
}

#[derive(Serialize, Deserialize)]
struct PaymentResponse {
    status: String,
    txn_id: String,
    message: String,
}

async fn send_payment(api_url: &str) -> Result<PaymentResponse, reqwest::Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let payload = UpiPaymentRequest {
        payer_upi: "venkat@upi".to_string(),
        payee_upi: "merchant@upi".to_string(),
        amount: 299.99,
        txn_note: "Test Payment".to_string(),
        merchant_id: "MERCHANT123456".to_string(),
    };

    let res = client
        .post(api_url)
        .header("Authorization", "Bearer test_token")
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let data: PaymentResponse = res.json().await?;
        Ok(data)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

#[tokio::test]
async fn test_payment_success() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .header("authorization", "Bearer test_token");

        then.status(200)
            .header("Content-Type", "application/json")
            .json_body_obj(&PaymentResponse {
                status: "SUCCESS".into(),
                txn_id: "TXN001".into(),
                message: "Payment successful".into(),
            });
    });

    let result = send_payment(&server.url("/")).await.unwrap();

    assert_eq!(result.status, "SUCCESS");
    assert_eq!(result.txn_id, "TXN001");
    assert_eq!(result.message, "Payment successful");

    mock.assert();
}

#[tokio::test]
async fn test_payment_unauthorized() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/");
        then.status(401)
            .header("Content-Type", "application/json")
            .body(r#"{"error":"Unauthorized"}"#);
    });

    let result = send_payment(&server.url("/")).await;

    assert!(result.is_err());
    mock.assert();
}
