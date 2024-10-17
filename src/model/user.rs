use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub public_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
