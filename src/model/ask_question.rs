use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AskQuestion {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
