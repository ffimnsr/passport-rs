use teloxide::{
    dispatching::dialogue::GetChatId, prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode}, utils
};
use url::Url;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use axum::{response::IntoResponse, routing::get, Router, http::StatusCode};
use tokio::{net::TcpListener, signal};

async fn health_check() -> &'static str {
    "I'm alive!"
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let app = Router::new()
        .route("/health", get(health_check))
        .fallback(handler_404);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    log::debug!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // let bot = Bot::from_env();

    // let handler = dptree::entry()
    //     .inspect(|u: Update| {
    //         log::info!("Update {u:#?}");
    //     })
    //     .branch(Update::filter_message().endpoint(message_handler));
    //     // .branch(
    //     //     Update::filter_chat_member()
    //     //         .branch(
    //     //             dptree::filter(|m: ChatMemberUpdated| {
    //     //                 m.old_chat_member.is_left() && m.new_chat_member.is_member()
    //     //             })
    //     //             .endpoint(new_chat_member),
    //     //         )
    //     //         .branch(
    //     //             dptree::filter(|m: ChatMemberUpdated| {
    //     //                 m.old_chat_member.is_member() && m.new_chat_member.is_left()
    //     //             })
    //     //             .endpoint(left_chat_member),
    //     //         )
    //     // );

    // Dispatcher::builder(bot, handler)
    //     .enable_ctrlc_handler()
    //     .build()
    //     .dispatch()
    //     .await;

    Ok(())
}

// async fn message_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let keyboard: InlineKeyboardMarkup = make_options();
//     // bot.send_message("@jobsesame".to_string(), "Debian versions:")
//     let image_url = Url::from_str("https://upload.wikimedia.org/wikipedia/commons/thumb/0/02/Stack_Overflow_logo.svg/200px-Stack_Overflow_logo.svg.png").unwrap();
//     let company_name = utils::markdown::bold("Company");
//     let company_one_liner = utils::markdown::escape("Company one-liner.");
//     let job_position = utils::markdown::bold("Software Engineer");
//     let message = indoc::formatdoc! {"
//         {company_name} is hiring\\!

//         {company_one_liner}

//         Position: {job_position}

//         Apply on the company's website or directly through JobSesame App 👇\\.

//         Subscribe to [Remote Jobs by Sesame](https://t.me/jobsesame) to not miss future updates\\!
//     "};

//     let image = InputFile::url(image_url);

//     bot.send_photo(msg.chat.id, image)
//         .caption(message)
//         .parse_mode(ParseMode::MarkdownV2)
//         .reply_markup(keyboard)
//         .await?;
//     Ok(())
// }

// fn make_options() -> InlineKeyboardMarkup {
//     let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

//     let debian_versions = [
//         "Buzz", "Rex", "Bo", "Hamm",
//     ];

//     let uri = Url::from_str("https://t.me/@JobSesameBot").unwrap();
//     for versions in debian_versions.chunks(2) {
//         let row = versions
//             .iter()
//             .map(|&version| InlineKeyboardButton::url(version.to_owned(), uri.to_owned()))
//             .collect();

//         keyboard.push(row);
//     }

//     InlineKeyboardMarkup::new(keyboard)
// }
