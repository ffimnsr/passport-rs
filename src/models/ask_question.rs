use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct AskQuestion {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
