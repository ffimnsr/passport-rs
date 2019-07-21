use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawalRequest {
    pub id: i32,
    pub requested_by_id: i32,
    pub approved_by_id: i32,
    pub bank_account_id: i32,
    pub amount: i32,
    pub reference_no: String,
    pub approved_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
