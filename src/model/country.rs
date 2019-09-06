use std::convert::From;

use actix::prelude::*;
use chrono::NaiveDateTime;
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub idd_code: String,
    pub currency: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Country {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_owned(),
            code: "".to_owned(),
            idd_code: "".to_owned(),
            currency: "".to_owned(),
            status: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl Country {
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for Country {
    type Result = FieldResult<Country>;
}

impl From<Row> for Country {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            code: row.get(2),
            idd_code: row.get(3),
            currency: row.get(4),
            status: row.get(5),
            created_at: row.get(6),
            updated_at: row.get(7),
        }
    }
}

impl From<&Row> for Country {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            code: row.get(2),
            idd_code: row.get(3),
            currency: row.get(4),
            status: row.get(5),
            created_at: row.get(6),
            updated_at: row.get(7),
        }
    }
}

impl Handler<Country> for Repo {
    type Result = FieldResult<Country>;

    fn handle(&mut self, _msg: Country, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.country", &[]).unwrap();
        let results: Vec<Country> = rows.iter().map(Country::from).collect::<Vec<Country>>();

        if results.is_empty() {
            Ok(Country::default())
        } else {
            Ok(results[0].clone())
        }
    }
}

pub struct Countries;

impl Message for Countries {
    type Result = FieldResult<Vec<Country>>;
}

impl Handler<Countries> for Repo {
    type Result = FieldResult<Vec<Country>>;

    fn handle(&mut self, _msg: Countries, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.country", &[]).unwrap();
        let results: Vec<Country> = rows.iter().map(Country::from).collect::<Vec<Country>>();

        Ok(results.clone())
    }
}
