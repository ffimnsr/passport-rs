pub mod country;
pub mod job;
pub mod organization;
pub mod user;
pub mod user_leaderboard_rank;
pub mod work_function;
pub mod work_industry;

pub use country::Country;
pub use job::Job;
pub use job::NewJob;
pub use job::UpdateJob;
pub use organization::Organization;
pub use work_function::WorkFunction;
pub use work_industry::WorkIndustry;

// Escape special characters in input strings
pub fn clean_input(input: &str) -> String {
    input
        .replace("'", "''")
        .replace("\"", "\"\"")
        .replace("\\", "\\\\")
        .replace("%", "\\%")
        .replace("_", "\\_")
}

#[derive(Debug)]
pub struct PaginationParams {
    pub limit: Option<isize>,
    pub offset: Option<isize>,
}

#[allow(dead_code)]
impl PaginationParams {
    pub fn new(limit: isize, offset: isize) -> Self {
        Self {
            limit: Some(limit),
            offset: Some(offset),
        }
    }

    pub fn paginate_query<S: Into<String>>(&self, query: S) -> String {
        let mut query = query.into();
        if let Some(limit) = self.limit {
            let payload = format!(" LIMIT {limit}");
            query.push_str(&payload);
        }

        if let Some(offset) = self.offset {
            let payload = format!(" OFFSET {offset}");
            query.push_str(&payload);
        }

        query
    }
}
