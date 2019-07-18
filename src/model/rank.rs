use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
