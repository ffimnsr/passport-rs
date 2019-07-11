use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub idd_code: String,
    pub currency: String,
    pub status: String,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
