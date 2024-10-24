use chrono::{DateTime, Utc};
use cuid2::cuid;
use serde::{Deserialize, Serialize};
use sqlx::query_builder::QueryBuilder;
use sqlx::PgConnection;
use sqlx::Postgres;
use validator::Validate;
use fake::Dummy;
use fake::faker::company::en::Profession;
use fake::faker::lorem::en::Paragraph;


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

#[derive(Debug, Validate, Dummy, Deserialize, Serialize)]
pub struct NewJob {
    #[dummy(faker = "Profession()")]
    #[validate(length(min = 5, max = 300))]
    pub title: String,
    #[dummy(faker = "Paragraph(1..40)")]
    #[validate(length(min = 10))]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateJob {
    pub title: Option<String>,
    pub description: Option<String>,
    pub industry_id: Option<i32>,
    pub country_id: Option<i32>,
    pub organization_id: Option<i64>,
    pub work_experience_level: Option<ExperienceLevel>,
    pub work_contract_type: Option<ContractType>,
    pub salary: Option<SalaryDetail>,
    pub has_timetracker: Option<bool>,
    pub is_remote: Option<bool>,
    pub is_featured: Option<bool>,
    pub status: Option<JobStatus>,
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

#[allow(dead_code)]
impl Job {
    #[allow(unused_variables)]
    pub async fn list(conn: &mut PgConnection, opt: Option<PaginationParams>) -> sqlx::Result<Jobs> {
        let query = "SELECT * FROM jobs".to_string();
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let data = sqlx::query_as::<_, Job>(&query).fetch_all(conn).await?;
        Ok(data)
    }

    pub async fn search(
        conn: &mut PgConnection,
        query: &str,
        opt: Option<PaginationParams>,
    ) -> sqlx::Result<Jobs> {
        let cleaned_input = clean_input(query);
        let query = format!("SELECT * FROM jobs WHERE fts @@ to_tsquery('english', '{cleaned_input}')");
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let data = sqlx::query_as::<_, Job>(&query).fetch_all(conn).await?;
        Ok(data)
    }

    pub async fn get_with_id(conn: &mut PgConnection, id: &str) -> sqlx::Result<Job> {
        let data = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(data)
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

    pub async fn update_with_id(conn: &mut PgConnection, id: &str, data: UpdateJob) -> sqlx::Result<()> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE jobs SET ");
        let mut separated = query_builder.separated(", ");

        macro_rules! append_field {
            ($field_name:ident) => {
                if let Some(value) = data.$field_name {
                    let q = format!(concat!(stringify!($field_name), " = "));
                    separated.push(q);
                    separated.push_bind_unseparated(value);
                }
            };
        }

        append_field!(title);
        append_field!(description);
        append_field!(industry_id);
        append_field!(country_id);
        append_field!(organization_id);
        append_field!(work_experience_level);
        append_field!(work_contract_type);
        append_field!(salary);
        append_field!(has_timetracker);
        append_field!(is_remote);
        append_field!(is_featured);
        append_field!(status);

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        // let s = query_builder.into_sql();
        // log::info!("Query: {:?}", s);

        let query = query_builder.build();
        query.execute(conn).await?;
        Ok(())
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
    use sqlx::error::Error;
    use sqlx::postgres::PgQueryResult;
    use sqlx::{Executor, PgPool};

    use super::*;

    async fn setup_job_table(conn: &mut PgConnection) -> Result<PgQueryResult, Error> {
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

            ALTER TABLE jobs
            ADD COLUMN fts tsvector
            GENERATED ALWAYS
            AS (to_tsvector('english', description || ' ' || title)) STORED;

            CREATE INDEX jobs_fts_idx ON jobs USING gin (fts);

            INSERT INTO jobs (id, title, description) VALUES ('foobar', 'foo', 'bar');
        "};

        conn.execute(query).await
    }

    #[sqlx::test(migrations = false)]
    async fn test_list(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        setup_job_table(&mut conn).await?;
        let foo = Job::list(&mut conn, None).await?;
        assert_eq!(foo.len(), 1);
        assert_eq!(foo[0].title, "foo");
        assert_eq!(foo[0].description, "bar");
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn test_search(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        setup_job_table(&mut conn).await?;
        let foo = Job::search(&mut conn, "foo", None).await?;
        assert_eq!(foo.len(), 1);
        assert_eq!(foo[0].title, "foo");
        assert_eq!(foo[0].description, "bar");
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn test_get_with_id(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        setup_job_table(&mut conn).await?;
        let foo = Job::get_with_id(&mut conn, "foobar").await?;
        assert_eq!(foo.title, "foo");
        assert_eq!(foo.description, "bar");
        Ok(())
    }
}
