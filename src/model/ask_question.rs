use std::convert::From;

use actix::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;

use super::{Repo, Connection};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct AskQuestion {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for AskQuestion {
    fn default() -> Self {
        Self {
            id: 0,
            question: "".to_owned(),
            answer: "".to_owned(),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl AskQuestion {
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for AskQuestion {
    type Result = FieldResult<AskQuestion>;
}

impl From<Row> for AskQuestion {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            question: row.get(1),
            answer: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }
}

impl From<&Row> for AskQuestion {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            question: row.get(1),
            answer: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }
}

impl Handler<AskQuestion> for Repo {
    type Result = FieldResult<AskQuestion>;

    fn handle(&mut self, _msg: AskQuestion, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.ask_question", &[]).unwrap();
        let results: Vec<AskQuestion> = rows.iter().map(AskQuestion::from).collect::<Vec<AskQuestion>>();

        if results.is_empty() {
            Ok(AskQuestion::default())
        } else {
            Ok(results[0].clone())
        }
    }
}

pub struct AskQuestions;

impl Message for AskQuestions {
    type Result = FieldResult<Vec<AskQuestion>>;
}

impl Handler<AskQuestions> for Repo {
    type Result = FieldResult<Vec<AskQuestion>>;

    fn handle(&mut self, _msg: AskQuestions, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.ask_question", &[]).unwrap();
        let results: Vec<AskQuestion> = rows.iter().map(AskQuestion::from).collect::<Vec<AskQuestion>>();

        Ok(results.clone())
    }
}
