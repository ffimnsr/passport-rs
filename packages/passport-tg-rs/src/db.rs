use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> sqlx::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

#[allow(dead_code)]
async fn notify(pool: &PgPool, s: &str) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        SELECT pg_notify(chan, payload)
        FROM (VALUES ('job_announcements', $1)) v(chan, payload)
        "#,
    )
    .bind(s)
    .execute(pool)
    .await?;

    Ok(())
}
