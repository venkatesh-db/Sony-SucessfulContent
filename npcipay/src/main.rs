
use reqwest::{Client, Error};
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct PaymentResponse {
    status: String,
    txn_id: String,
    message: String,
}

async fn make_payment_request() -> Result<(), Error> {
    
    let api_url = "https://npci-com.free.beeceptor.com";

    let payment_payload = UpiPaymentRequest {
        payer_upi: "venkat@upi".to_string(),
        payee_upi: "merchant@upi".to_string(),
        amount: 299.99,
        txn_note: "Testing NPCI payment".to_string(),
        merchant_id: "MERCHANT123456".to_string(),
    };

    println!(
        "Sending payload:\n{}",
        serde_json::to_string_pretty(&payment_payload).unwrap()
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer test_token_123")
        .json(&payment_payload)
        .send()
        .await?;

    let status = response.status();

    if status.is_success() {
        match response.json::<PaymentResponse>().await {
            Ok(payment_response) => {
                println!(
                    "[SUCCESS] Payment ID: {}, Message: {}",
                    payment_response.txn_id, payment_response.message
                );
            }
            Err(_) => {
                println!("[SUCCESS] Received success status but could not parse JSON.");
            }
        }
    } else {
        let text = response.text().await.unwrap_or_default();
        eprintln!("[ERROR] Status: {}, Response: {}", status, text);
    }

    Ok(())
}

#[tokio::main]
async fn main() {

    match make_payment_request().await {
        Ok(_) => println!("✅ Payment request completed."),
        Err(e) => eprintln!("❌ Request failed: {:?}", e),
    }
}

