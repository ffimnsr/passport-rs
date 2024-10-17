use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub idd_code: String,
    pub currency: String,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
