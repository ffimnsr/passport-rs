use actix::prelude::*;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

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
pub mod withdrawal_request;
pub mod work_function;
