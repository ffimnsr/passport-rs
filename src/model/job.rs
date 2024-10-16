use chrono::{DateTime, Utc};
use cuid2::cuid;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use validator::Validate;

use super::clean_input;
use super::PaginationParams;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_experience_level", rename_all = "snake_case")]
pub enum ExperienceLevel {
    Intern,
    EntryLevel,
    MidLevel,
    SeniorLevel,
    Executive,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_contract_type", rename_all = "snake_case")]
pub enum ContractType {
    FullTime,
    PartTime,
    FreelanceContract,
    FixedTermContract,
    ZeroHourContract,
    Internship,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "salary_timeframe", rename_all = "snake_case")]
pub enum SalaryTimeframe {
    Hourly,
    Daily,
    Weekly,
    SemiMonthly,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "salary_detail")]
pub struct SalaryDetail {
    upper_limit: String,
    lower_limit: String,
    currency: String,
    timeframe: SalaryTimeframe,
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
    pub id: String,
    pub title: String,
    pub description: String,
    pub industry_id: i32,
    pub country_id: i32,
    pub organization_id: i64,
    pub work_experience_level: ExperienceLevel,
    pub work_contract_type: ContractType,
    pub salary: Option<SalaryDetail>,
    pub has_timetracker: bool,
    pub is_remote: bool,
    pub is_featured: bool,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Jobs = Vec<Job>;

#[allow(dead_code, unused_variables)]
impl Job {
    #[allow(unused_variables)]
    pub async fn list(conn: &mut PgConnection, opt: Option<PaginationParams>) -> sqlx::Result<Jobs> {
        let query = "SELECT * FROM jobs".to_string();
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let jobs = sqlx::query_as::<_, Job>(&query).fetch_all(conn).await?;
        Ok(jobs)
    }

    pub async fn search(
        conn: &mut PgConnection,
        query: &str,
        opt: Option<PaginationParams>,
    ) -> sqlx::Result<Jobs> {
        let cleaned_input = clean_input(query);
        let query = format!("SELECT * FROM jobs WHERE fts @@ to_tsquery('english', '{cleaned_input}')");
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);

        let jobs = sqlx::query_as::<_, Job>(&query).fetch_all(conn).await?;
        Ok(jobs)
    }

    pub async fn get_with_id(conn: &mut PgConnection, id: &str) -> sqlx::Result<Job> {
        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(job)
    }

    pub async fn insert(conn: &mut PgConnection, data: NewJob) -> sqlx::Result<String> {
        let row: (String,) =
            sqlx::query_as("INSERT INTO jobs (id, title, description) VALUES ($1, $2, $3) RETURNING id")
                .bind(cuid())
                .bind(data.title)
                .bind(data.description)
                .fetch_one(conn)
                .await?;
        Ok(row.0)
    }

    pub async fn delete_with_id(conn: &mut PgConnection, id: &str) -> sqlx::Result<()> {
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
            CREATE TYPE work_experience_level AS ENUM (
                'intern',
                'entry_level',
                'mid_level',
                'senior_level',
                'executive'
            );

            CREATE TYPE work_contract_type AS ENUM (
                'full_time',
                'part_time',
                'freelance_contract',
                'fixed_term_contract',
                'zero_hour_contract',
                'internship'
            );

            CREATE TYPE salary_timeframe AS ENUM (
                'hourly',
                'daily',
                'weekly',
                'semi_monthly',
                'monthly',
                'quarterly',
                'annually'
            );

            CREATE TYPE salary_detail AS (
                upper_limit TEXT,
                lower_limit TEXT,
                currency VARCHAR(10),
                timeframe salary_timeframe
            );

            CREATE TEMP TABLE IF NOT EXISTS jobs (
                id VARCHAR(24) PRIMARY KEY,
                title VARCHAR(300) NOT NULL,
                description TEXT NOT NULL,
                industry_id INT DEFAULT 1000,
                country_id INT DEFAULT 1,
                organization_id BIGINT DEFAULT 1,
                work_experience_level work_experience_level DEFAULT 'intern',
                work_contract_type work_contract_type DEFAULT 'full_time',
                salary salary_detail,
                has_timetracker BOOL DEFAULT FALSE,
                is_remote BOOL DEFAULT TRUE,
                is_featured BOOL DEFAULT FALSE,
                status SMALLINT DEFAULT 1,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );

            INSERT INTO jobs (id, title, description) VALUES ('foobar', 'foo', 'bar');
        "};

        conn.execute(query).await?;

        let foo = Job::list(&mut conn, None).await?;
        assert_eq!(foo.len(), 1);
        assert_eq!(foo[0].title, "foo");
        assert_eq!(foo[0].description, "bar");
        Ok(())
    }
}
