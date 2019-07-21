use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub employer_id: i32,
    pub code: String,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectClue {
    pub id: i32,
    pub project_id: i32,
    pub description: String,
    pub repo_http_url: String,
    pub repo_ssh_url: String,
    pub repo_web_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMember {
    pub id: i32,
    pub project_id: i32,
    pub talent_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIssue {
    pub id: i32,
    pub project_id: i32,
    pub assigned_to_id: i32,
    pub reported_by_id: i32,
    pub code: String,
    pub description: String,
    pub assigned_at: NaiveDateTime,
    pub reported_by: NaiveDateTime,
    pub priority: i32,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
