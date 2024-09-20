// pub mod ask_question;
pub mod country;
pub mod job;
// pub mod industry;
// pub mod project;
// pub mod rank;
// pub mod user;
// pub mod oauth_identity;
// pub mod bank_account;
// pub mod withdrawal_request;
// pub mod work_function;

// pub use ask_question::{AskQuestions, AskQuestion};
// pub use country::{Countries, Country};
pub use job::Job;
// pub use industry::{Industries, Industry};
// pub use project::{Project};
// pub use rank::{Ranks, Rank};
// pub use user::{User};
// pub use oauth_identity::{OauthIdentity};
// pub use bank_account::{BankAccount};
// pub use withdrawal_request::{WithdrawalRequest};
// pub use work_function::{WorkFunctions, WorkFunction};

// pub trait Repo {
//     fn all<T>() -> sqlx::Result<Vec<T>>;
//     fn insert<T>(data: T) -> sqlx::Result<i64>;
//     fn delete_with_id<T>(id: i64) -> sqlx::Result<()>;
// }
