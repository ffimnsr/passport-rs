use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Industry {
    pub id: i32,
    pub name: String,
    pub status: i8,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewIndustry {
    pub id: i32,
    pub name: String,
    pub status: i8,
}
