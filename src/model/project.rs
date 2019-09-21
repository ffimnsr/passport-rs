use std::convert::From;

use actix::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
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

impl Default for Project {
    fn default() -> Self {
        Self {
            id: 0,
            employer_id: 0,
            code: String::new(),
            name: String::new(),
            start_date: NaiveDate::from_ymd(1970, 1, 1),
            end_date: NaiveDate::from_ymd(1970, 1, 1),
            status: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl Project {
    #[allow(dead_code)]
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for Project {
    type Result = FieldResult<Project>;
}

impl From<Row> for Project {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            employer_id: row.get(1),
            code: row.get(2),
            name: row.get(3),
            start_date: row.get(4),
            end_date: row.get(5),
            status: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        }
    }
}

impl From<&Row> for Project {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            employer_id: row.get(1),
            code: row.get(2),
            name: row.get(3),
            start_date: row.get(4),
            end_date: row.get(5),
            status: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        }
    }
}

impl Handler<Project> for Repo {
    type Result = FieldResult<Project>;

    fn handle(&mut self, _msg: Project, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.projects", &[]).unwrap();
        let results: Vec<Project> = rows.iter()
            .map(Project::from)
            .collect::<Vec<Project>>();

        if results.is_empty() {
            Ok(Project::default())
        } else {
            Ok(results[0].clone())
        }
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
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

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMember {
    pub id: i32,
    pub project_id: i32,
    pub talent_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
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
