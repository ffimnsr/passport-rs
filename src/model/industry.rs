use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Industry {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewIndustry {
    pub id: i32,
    pub name: String,
    pub status: i32,
}
