use std::convert::From;

use actix::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;

use super::{Repo, Connection};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "A structure that defines user rank")]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Rank {
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

impl Rank {
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for Rank {
    type Result = FieldResult<Rank>;
}

impl From<Row> for Rank {
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

impl From<&Row> for Rank {
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

impl Handler<Rank> for Repo {
    type Result = FieldResult<Rank>;

    fn handle(&mut self, _msg: Rank, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.rank", &[]).unwrap();
        let results: Vec<Rank> = rows.iter().map(Rank::from).collect::<Vec<Rank>>();

        if results.is_empty() {
            Ok(Rank::default())
        } else {
            Ok(results[0].clone())
        }
    }
}

pub struct Ranks;

impl Message for Ranks {
    type Result = FieldResult<Vec<Rank>>;
}

impl Handler<Ranks> for Repo {
    type Result = FieldResult<Vec<Rank>>;

    fn handle(&mut self, _msg: Ranks, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.rank", &[]).unwrap();
        let results: Vec<Rank> = rows.iter().map(Rank::from).collect::<Vec<Rank>>();

        Ok(results.clone())
    }
}
