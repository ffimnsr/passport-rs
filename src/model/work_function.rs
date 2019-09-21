use std::convert::From;

use actix::prelude::*;
use chrono::NaiveDateTime;
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct WorkFunction {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for WorkFunction {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            status: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl WorkFunction {
    #[allow(dead_code)]
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for WorkFunction {
    type Result = FieldResult<WorkFunction>;
}

impl From<Row> for WorkFunction {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }
}

impl From<&Row> for WorkFunction {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }
}

impl Handler<WorkFunction> for Repo {
    type Result = FieldResult<WorkFunction>;

    fn handle(&mut self, _msg: WorkFunction, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.work_functions", &[]).unwrap();
        let results: Vec<WorkFunction> = rows.iter()
            .map(WorkFunction::from)
            .collect::<Vec<WorkFunction>>();

        if results.is_empty() {
            Ok(WorkFunction::default())
        } else {
            Ok(results[0].clone())
        }
    }
}

pub struct WorkFunctions;

impl Message for WorkFunctions {
    type Result = FieldResult<Vec<WorkFunction>>;
}

impl Handler<WorkFunctions> for Repo {
    type Result = FieldResult<Vec<WorkFunction>>;

    fn handle(&mut self, _msg: WorkFunctions, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.work_functions", &[]).unwrap();
        let results: Vec<WorkFunction> = rows.iter()
            .map(WorkFunction::from)
            .collect::<Vec<WorkFunction>>();

        Ok(results.clone())
    }
}
