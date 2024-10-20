use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use super::PaginationParams;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct WorkIndustry {
    pub id: i32,
    pub name: String,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type WorkIndustries = Vec<WorkIndustry>;

#[allow(dead_code)]
impl WorkIndustry {
    pub async fn list(conn: &mut PgConnection, opt: Option<PaginationParams>) -> sqlx::Result<WorkIndustries> {
        let query = "SELECT * FROM work_industries".to_string();
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let data = sqlx::query_as::<_, WorkIndustry>(&query).fetch_all(conn).await?;
        Ok(data)
    }

    pub async fn get_with_id(conn: &mut PgConnection, id: i64) -> sqlx::Result<WorkIndustry> {
        let data = sqlx::query_as::<_, WorkIndustry>("SELECT * FROM work_industries WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(data)
    }
}
