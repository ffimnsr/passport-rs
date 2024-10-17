use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use super::PaginationParams;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Organization {
    pub id: i64,
    pub public_id: String,
    pub name: String,
    pub is_verified: bool,
    pub is_featured: bool,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Organizations = Vec<Organization>;

#[allow(dead_code)]
impl Organization {
    pub async fn list(conn: &mut PgConnection, opt: Option<PaginationParams>) -> sqlx::Result<Organizations> {
        let query = "SELECT * FROM organizations".to_string();
        let query = opt.map(|x| x.paginate_query(query.clone())).unwrap_or(query);
        let jobs = sqlx::query_as::<_, Organization>(&query).fetch_all(conn).await?;
        Ok(jobs)
    }

    pub async fn get_with_id(conn: &mut PgConnection, id: i64) -> sqlx::Result<Organization> {
        let job = sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await?;
        Ok(job)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use sqlx::{Executor, PgPool};
    use sqlx::postgres::PgQueryResult;
    use sqlx::error::Error;

    use super::*;

    async fn setup_organization_table(conn: &mut PgConnection) -> Result<PgQueryResult, Error> {
        let query = indoc! {"
            CREATE TABLE IF NOT EXISTS organizations (
                id BIGSERIAL PRIMARY KEY,
                public_id TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                is_verified BOOL DEFAULT FALSE,
                is_featured BOOL DEFAULT FALSE,
                status SMALLINT DEFAULT 1,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );

            INSERT INTO organizations (public_id, name) VALUES ('foobar', 'foo');
        "};

        conn.execute(query).await
    }

    #[sqlx::test(migrations = false)]
    async fn test_list(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        setup_organization_table(&mut conn).await?;
        let foo = Organization::list(&mut conn, None).await?;
        assert_eq!(foo.len(), 1);
        assert_eq!(foo[0].public_id, "foobar");
        assert_eq!(foo[0].name, "foo");
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn test_get_with_id(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        setup_organization_table(&mut conn).await?;
        let foo = Organization::get_with_id(&mut conn, 1).await?;
        assert_eq!(foo.public_id, "foobar");
        assert_eq!(foo.name, "foo");
        Ok(())
    }
}
