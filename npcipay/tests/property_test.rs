
use proptest::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UpiPaymentRequest {
    payer_upi: String,
    payee_upi: String,
    amount: f64,
    txn_note: String,
    merchant_id: String,
}

// Example property: Amount must always be > 0
proptest! {
    #[test]
    fn payment_amount_is_positive(amount in 0.01f64..100000.0) {
        let request = UpiPaymentRequest {
            payer_upi: "venkat@upi".to_string(),
            payee_upi: "merchant@upi".to_string(),
            amount,
            txn_note: "Proptest UPI".to_string(),
            merchant_id: "MERCHANT123".to_string(),
        };

        // Property: amount should be greater than zero
        prop_assert!(request.amount > 0.0);
    }
}


