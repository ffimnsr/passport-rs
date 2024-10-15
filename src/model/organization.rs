use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub is_verified: bool,
    pub is_featured: bool,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
