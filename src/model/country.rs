use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub idd_code: String,
    pub currency: String,
    pub status: i32,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
