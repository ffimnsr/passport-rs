use anyhow::Context;
use chrono::{DateTime, Utc};
use passport_tg::{db, model::{job::DetailedJob, pubsub_job_announcement::{PubsubJobAnnouncement, PubsubMessageStatus, SaveMessageIdProps}}};
use sqlx::{postgres::PgListener, PgPool};
use teloxide::{
    prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode}, utils
};
use url::Url;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use axum::{http::StatusCode, response::{IntoResponse, Response}, routing::get, Router};
use tokio::{net::TcpListener, signal, sync::Mutex};
use std::{env, str::FromStr};
use std::sync::Arc;

struct AnnouncerApp {
    messages: Arc<Mutex<Vec<String>>>,
    pool: PgPool,
    bot: Bot,
}

impl AnnouncerApp {
    fn new(pool: PgPool, bot: Bot) -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            pool,
            bot,
        }
    }

    async fn run(&self, mut listener: PgListener) -> anyhow::Result<()> {
        log::info!("Starting announcer server...");
        let messages = self.messages.clone();
        tokio::spawn(async move {
            while let Ok(msg) = listener.recv().await {
                messages.lock().await.push(msg.payload().to_string());
            }
        });

        let mut pending_interval = tokio::time::interval(tokio::time::Duration::from_secs(20));
        loop {
            tokio::select! {
                _ = shutdown_signal() => {
                    log::info!("Shutting down announcer server...");
                    break;
                }
                _ = pending_interval.tick() => {
                    log::info!("Checking for some pending messages...");
                    if let Ok(announcement) = self.get_pending_announcement().await {
                        let mut messages = self.messages.lock().await;
                        if messages.contains(&announcement.job_id) {
                            continue;
                        }

                        messages.push(announcement.job_id);
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(3)) => {
                    log::info!("Checking for new messages...");
                    let messages: Vec<String> = self.messages
                        .lock()
                        .await
                        .iter()
                        .map(|m| m.to_owned())
                        .collect();

                    for job_id in messages {
                        log::info!("Message: {}", job_id);
                        let job = self.find_job_details(&job_id)
                            .await
                            .context(format!("No job found with id: {job_id:#?}"))?;
                        let message_id = self.message_channel(&job).await?;
                        self.save_message_id(&job_id, message_id).await?;
                    }

                    self.messages.lock().await.clear();
                }
            }
        }

        Ok(())
    }

    async fn find_job_details(&self, job_id: &str) -> anyhow::Result<DetailedJob> {
        let job = DetailedJob::get_with_id(&self.pool, job_id).await?;
        Ok(job)
    }

    async fn save_message_id(&self, job_id: &str, message_id: i32) -> anyhow::Result<()> {
        PubsubJobAnnouncement::save_message_id(&self.pool, job_id, SaveMessageIdProps {
            message_id: Some(message_id),
            message_status: Some(PubsubMessageStatus::Delivered),
            delivered_at: DateTime::from(Utc::now()).into(),
        }).await?;
        Ok(())
    }

    async fn get_pending_announcement(&self) -> anyhow::Result<PubsubJobAnnouncement> {
        let announcement = PubsubJobAnnouncement::get_pending_announcement(&self.pool).await?;
        PubsubJobAnnouncement::increment_announcement_delivery_attempt(&self.pool, &announcement.job_id).await?;
        Ok(announcement)
    }

    async fn message_channel(&self, job: &DetailedJob) -> anyhow::Result<i32> {
        let keyboard: InlineKeyboardMarkup = make_post_options(&job.id);

        let channel_id = "@jobsesame".to_string();
        let channel_url = utils::markdown::link("https://t.me/jobsesame", "Remote Jobs by Sesame");
        let company_image_url = Url::from_str(&job.organization_image_url()).unwrap();
        let company_name = utils::markdown::bold(&job.organization_name);
        let company_one_liner = utils::markdown::escape(&job.organization_description());
        let company_url = utils::markdown::link(&job.organization_website_url(), "company's website");
        let job_position = utils::markdown::bold(&job.title());
        let job_location = utils::markdown::escape(&job.is_remote_text());
        let job_contract_type = utils::markdown::escape(&job.contract_type());
        let message = indoc::formatdoc! {"
            👉 {company_name} is hiring\\! 👈

            {company_one_liner}

            Position: {job_position}

            Location: 📍 {job_location}
            Type: 🕒 {job_contract_type}

            Apply directly on the {company_url} or through JobSesame WebApp 👇\\.

            Stay ahead in your career\\! Subscribe to {channel_url} and never miss an update\\! 🚀
        "};

        let image = InputFile::url(company_image_url);

        let post_msg = self.bot.send_photo(channel_id, image)
            .caption(message)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
        Ok(post_msg.id.0)
    }
}

fn action_url(url: &str, job_id: &str) -> Url {
    let temp = format!("{url}_{job_id}");
    convert_to_url(&temp)
}

fn convert_to_url(s: &str) -> Url {
    Url::from_str(s)
        .unwrap_or_else(|_| Url::parse("https://t.me/@JobSesameBot").unwrap())
}

fn make_post_options(job_id: &str) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let action_buttons = [
        ("See Details 🔍", action_url("https://t.me/@JobSesameBot/app?startapp=view", job_id)),
        ("Save ➕", action_url("https://t.me/@JobSesameBot?start=save", job_id)),
        ("Forward ⏩", action_url("https://t.me/@JobSesameBot?start=forward", job_id)),
        ("Help ❓", action_url("https://t.me/@JobSesameBot/app?start=help", job_id)),
        ("Apply Now ✅", action_url("https://t.me/@JobSesameBot/app?startapp=apply", job_id)),
    ];

    for buttons in action_buttons.chunks(2) {
        let row = buttons
            .iter()
            .map(|(name, url)| InlineKeyboardButton::url(name.to_owned(), url.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

async fn health_check() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(())
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn start_health_check_server(pool: &PgPool) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .fallback(handler_404)
        .with_state(pool.to_owned());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    log::debug!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,announcer=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let database_url = env::var("DATABASE_URL").context("Failed to get DATABASE_URL env variable")?;
    let pool = db::init_pool(&database_url).await.context("Failed to create postges pool")?;

    let mut listener = PgListener::connect(&database_url).await?;
    listener.listen("job_announcements").await?;

    let bot = Bot::from_env();

    let announcer_server = async {
        let app = AnnouncerApp::new(pool.clone(), bot);
        let res = app.run(listener).await;

        if let Err(e) = res {
            log::error!("Error: {:?}", e);
        }
    };

    let _ = tokio::join!(announcer_server, start_health_check_server(&pool));
    Ok(())
}
