use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct AskQuestion {
    pub id: i32,
    pub requested_by_id: i32,
    pub approved_by_id: i32,
    pub bank_account_id: i32,
    pub amount: i64,
    pub reference_no: String,
    pub approved_at: DateTime<Utc>,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
