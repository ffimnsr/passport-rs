pub mod country;
pub mod job;
pub mod organization;
pub mod user;
pub mod user_leaderboard_rank;
pub mod work_function;
pub mod work_industry;

pub use job::Job;

// Escape special characters in input strings
pub fn clean_input(input: &str) -> String {
    input
        .replace("'", "''")
        .replace("\"", "\"\"")
        .replace("\\", "\\\\")
        .replace("%", "\\%")
        .replace("_", "\\_")
}
