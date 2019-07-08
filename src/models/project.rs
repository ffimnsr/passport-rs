use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub employer_id: i32,
    pub code: String,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: i8,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectClue {
    pub id: i32,
    pub project_id: i32,
    pub description: String,
    pub repo_http_url: String,
    pub repo_ssh_url: String,
    pub repo_web_url: String,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMember {
    pub id: i32,
    pub project_id: i32,
    pub talent_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIssue {
    pub id: i32,
    pub project_id: i32,
    pub assigned_to_id: i32,
    pub reported_by_id: i32,
    pub code: String,
    pub description: String,
    pub assigned_at: DateTime<Utc>,
    pub reported_by: DateTime<Utc>,
    pub priority: i8,
    pub status: i8,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
