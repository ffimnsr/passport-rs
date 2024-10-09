use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Organization {
    pub id: i32,
    pub public_id: String,
    pub name: String,
    pub is_verified: bool,
    pub is_featured: bool,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
