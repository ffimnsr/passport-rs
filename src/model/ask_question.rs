use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct AskQuestion {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
