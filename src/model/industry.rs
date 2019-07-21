use std::convert::From;
use actix::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;

use super::{Repo, Connection};

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
#[graphql(description = "A structure that defines project industry")]
pub struct Industry {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Industry {
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Default for Industry {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_owned(),
            status: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl Message for Industry {
    type Result = FieldResult<Industry>;
}

impl From<Row> for Industry {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            created_at: NaiveDateTime::parse_from_str(row.get(3), "%Y-%m-%d %H:%M:%S%.f").unwrap(),
            updated_at: NaiveDateTime::parse_from_str(row.get(4), "%Y-%m-%d %H:%M:%S%.f").unwrap(),            
        }
    }
}

impl From<&Row> for Industry {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            created_at: NaiveDateTime::parse_from_str(row.get(3), "%Y-%m-%d %H:%M:%S%.f").unwrap(),
            updated_at: NaiveDateTime::parse_from_str(row.get(4), "%Y-%m-%d %H:%M:%S%.f").unwrap(),            
        }
    }
}

impl Handler<Industry> for Repo {
    type Result = FieldResult<Industry>;

    fn handle(&mut self, _msg: Industry, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();

        let rows: Vec<Row> = client.query("SELECT * FROM public.industry", &[]).unwrap();
        let _results = rows.iter().map(Industry::from).collect::<Vec<Industry>>();

        Ok(Industry::default())
    }
}

pub struct Industries;

impl Message for Industries {
    type Result = FieldResult<Vec<Industry>>;
}
