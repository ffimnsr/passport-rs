use actix::prelude::*;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

pub type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;
pub type Connection = r2d2::PooledConnection<PostgresConnectionManager<NoTls>>;

pub struct Repo(pub Pool);

impl Actor for Repo {
    type Context = SyncContext<Self>;
}

pub mod ask_question;
pub mod country;
pub mod industry;
pub mod project;
pub mod rank;
pub mod user;
pub mod oauth_identity;
pub mod bank_account;
pub mod withdrawal_request;
pub mod work_function;

pub use ask_question::{AskQuestions, AskQuestion};
pub use country::{Countries, Country};
pub use industry::{Industries, Industry};
pub use project::{Project};
pub use rank::{Ranks, Rank};
pub use user::{User};
pub use oauth_identity::{OauthIdentity};
pub use bank_account::{BankAccount};
pub use withdrawal_request::{WithdrawalRequest};
pub use work_function::{WorkFunctions, WorkFunction};
