use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use super::PaginationParams;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub public_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Users = Vec<User>;

#[allow(dead_code)]
impl User {
    pub async fn list(conn: &mut PgConnection, opt: Option<PaginationParams>) -> sqlx::Result<Users> {
        let query = "SELECT * FROM users".to_string();
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let data = sqlx::query_as::<_, User>(&query).fetch_all(conn).await?;
        Ok(data)
    }

    pub async fn get_with_id(conn: &mut PgConnection, id: i64) -> sqlx::Result<User> {
        let data = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(data)
    }
}
