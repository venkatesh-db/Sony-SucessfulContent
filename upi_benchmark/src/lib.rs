
#[derive(Debug)]
pub struct UpiPaymentRequest {
    pub payer_upi: String,
    pub payee_upi: String,
    pub amount: f64,
    pub txn_note: String,
    pub merchant_id: String,
}

pub fn create_upi_payload(payer: &str, payee: &str, amount: f64) -> UpiPaymentRequest {
    UpiPaymentRequest {
        payer_upi: payer.to_string(),
        payee_upi: payee.to_string(),
        amount,
        txn_note: "Testing from lib".to_string(),
        merchant_id: "MERCHANT123".to_string(),
    }
}
