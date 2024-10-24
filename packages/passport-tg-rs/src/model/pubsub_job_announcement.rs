use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pubsub_message_status", rename_all = "snake_case")]
pub enum PubsubMessageStatus {
    Pending,
    Delivered,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveMessageIdProps {
    pub message_id: Option<i32>,
    pub message_status: Option<PubsubMessageStatus>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PubsubJobAnnouncement {
    pub job_id: String,
    pub message_id: Option<i32>,
    pub message_status: PubsubMessageStatus,
    pub scheduled_delivery_at: DateTime<Utc>,
    pub delivery_attemps: i32,
    pub delivered_at: Option<DateTime<Utc>>,
}

impl PubsubJobAnnouncement {
    pub async fn get_pending_announcement(pool: &PgPool) -> sqlx::Result<PubsubJobAnnouncement> {
        let query = r#"
            SELECT
                job_id,
                message_id,
                message_status,
                scheduled_delivery_at,
                delivery_attemps,
                delivered_at
            FROM pubsub_job_announcements
            WHERE
                message_status = 'pending' AND
                scheduled_delivery_at <= NOW()
            ORDER BY created_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED;
        "#;

        let data = sqlx::query_as::<_, PubsubJobAnnouncement>(query)
            .fetch_one(pool)
            .await?;
        Ok(data)
    }

    pub async fn increment_announcement_delivery_attempt(pool: &PgPool, job_id: &str) -> sqlx::Result<()> {
        let query = r#"
            UPDATE pubsub_job_announcements
            SET delivery_attemps = delivery_attemps + 1
            WHERE job_id = $1;
        "#;

        sqlx::query(query)
            .bind(job_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn save_message_id(pool: &PgPool, job_id: &str, data: SaveMessageIdProps) -> sqlx::Result<()> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE pubsub_job_announcements SET ");
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

        append_field!(message_id);
        append_field!(message_status);
        append_field!(delivered_at);

        query_builder.push(" WHERE job_id = ");
        query_builder.push_bind(job_id);

        // let s = query_builder.into_sql();
        // log::info!("Query: {:?}", s);

        let query = query_builder.build();
        query.execute(pool).await?;
        Ok(())
    }
}
