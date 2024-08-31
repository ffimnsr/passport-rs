use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i16)]
pub enum ExperienceLevel {
    Intern = 1,
    Junior = 2,
    Mid = 3,
    Senior = 4,
    Lead = 5,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i16)]
pub enum WorkType {
    FullTime = 1,
    PartTime = 2,
    Contract = 3,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i16)]
pub enum JobStatus {
    Open = 1,
    Closed = 0,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct NewJob {
    #[validate(length(min = 5, max = 300))]
    pub title: String,
    #[validate(length(min = 10))]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub experience_level: Option<ExperienceLevel>,
    pub salary_upper_limit: Option<String>,
    pub salary_lower_limit: Option<String>,
    pub salary_currency: Option<String>,
    pub salary_timeframe: Option<String>,
    pub work_type: Option<WorkType>,
    pub has_timetracker: Option<bool>,
    pub status: Option<JobStatus>,
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

    pub async fn get_with_id(id: i32, conn: &mut PgConnection) -> sqlx::Result<Job> {
        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(job)
    }

    pub async fn insert(data: NewJob, conn: &mut PgConnection) -> sqlx::Result<i32> {
        let row: (i32,) =
            sqlx::query_as("INSERT INTO jobs (title, description) VALUES ($1, $2) RETURNING id")
                .bind(data.title)
                .bind(data.description)
                .fetch_one(conn)
                .await?;
        Ok(row.0)
    }

    pub async fn delete_with_id(id: i32, conn: &mut PgConnection) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM jobs WHERE id = $1")
            .bind(id)
            .execute(conn)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use sqlx::{Executor, PgPool};

    use super::*;

    #[sqlx::test(migrations = false)]
    async fn test_get_all_jobs(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        let query = indoc! {"
            CREATE TABLE IF NOT EXISTS jobs (
                id SERIAL PRIMARY KEY,
                title VARCHAR(300) NOT NULL UNIQUE,
                description TEXT NOT NULL,
                experience_level SMALLINT NOT NULL DEFAULT 1,
                salary_upper_limit TEXT,
                salary_lower_limit TEXT,
                salary_currency VARCHAR(10),
                salary_timeframe SMALLINT,
                work_type SMALLINT NOT NULL DEFAULT 1,
                has_timetracker BOOLEAN DEFAULT FALSE,
                status SMALLINT DEFAULT 1,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );

            INSERT INTO jobs (title, description) VALUES ('foo', 'bar');
        "};

        conn.execute(query).await?;

        let foo = Job::all(&mut conn).await?;
        assert_eq!(foo.len(), 1);
        assert_eq!(foo[0].title, "foo");
        assert_eq!(foo[0].description, "bar");
        Ok(())
    }
}
