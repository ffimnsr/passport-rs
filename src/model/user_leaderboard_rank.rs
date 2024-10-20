use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use super::PaginationParams;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Ranks = Vec<Rank>;

#[allow(dead_code)]
impl Rank {
    pub async fn list(conn: &mut PgConnection, opt: Option<PaginationParams>) -> sqlx::Result<Ranks> {
        let query = "SELECT * FROM ranks".to_string();
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let data = sqlx::query_as::<_, Rank>(&query).fetch_all(conn).await?;
        Ok(data)
    }

    pub async fn get_with_id(conn: &mut PgConnection, id: i64) -> sqlx::Result<Rank> {
        let data = sqlx::query_as::<_, Rank>("SELECT * FROM ranks WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(data)
    }
}
