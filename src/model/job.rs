use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewJob {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Option<i16>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Jobs = Vec<Job>;

impl Job {
    pub async fn all(conn: &mut PgConnection) -> sqlx::Result<Jobs> {
        let jobs = sqlx::query_as::<_, Job>("SELECT * FROM jobs").fetch(conn);
        let jobs = jobs
            .map(|j| j.expect("Error parsing job"))
            .collect::<Jobs>()
            .await;
        Ok(jobs)
    }

    pub async fn insert(data: NewJob, conn: &mut PgConnection) -> sqlx::Result<i64> {

        let row: (i64,) =
            sqlx::query_as("INSERT INTO jobs (title, description) VALUES ($1, $2) RETURNING id")
                .bind(data.title)
                .bind(data.description)
                .fetch_one(conn)
                .await?;
        Ok(row.0)
    }

    pub async fn delete_with_id(id: i64, conn: &mut PgConnection) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM jobs WHERE id = $1")
            .bind(id)
            .execute(conn)
            .await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use sqlx::{Any, AnyPool, PgPool, SqlitePool};

//     use super::*;

//     async fn it_works(pool: AnyPool) -> sqlx::Result<()> {
//       let mut conn = pool.acquire().await?;
//       let foo = Job::all(&mut conn).await?;
//       assert_eq!(foo.len(), 0);
//       Ok(())
//     }
// }
