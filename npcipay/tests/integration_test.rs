
use httpmock::prelude::*;
use reqwest::{Client};
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

#[tokio::test]
async fn integration_payment_success() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .header("authorization", "Bearer test_token");

        then.status(200)
            .header("Content-Type", "application/json")
            .json_body_obj(&PaymentResponse {
                status: "SUCCESS".into(),
                txn_id: "TXN789".into(),
                message: "Integration OK.".into(),
            });
    });

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let payload = UpiPaymentRequest {
        payer_upi: "venkat@upi".into(),
        payee_upi: "shop@upi".into(),
        amount: 149.75,
        txn_note: "Integration test".into(),
        merchant_id: "MID123".into(),
    };

    let res = client
        .post(&server.url("/"))
        .header("Authorization", "Bearer test_token")
        .json(&payload)
        .send()
        .await
        .unwrap();

    let parsed: PaymentResponse = res.json().await.unwrap();

    assert_eq!(parsed.status, "JAMON"); //"SUCCESS"
    assert_eq!(parsed.txn_id, "TXN789");

    mock.assert();
}
