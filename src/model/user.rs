use std::convert::From;

use actix::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub email_confirmed: bool,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            email: String::new(),
            email_confirmed: false,
            password_hash: String::new(),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl User {
    #[allow(dead_code)]
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for User {
    type Result = FieldResult<User>;
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            email: row.get(1),
            email_confirmed: row.get(2),
            password_hash: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
        }
    }
}

impl From<&Row> for User {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            email: row.get(1),
            email_confirmed: row.get(2),
            password_hash: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
        }
    }
}

impl Handler<User> for Repo {
    type Result = FieldResult<User>;

    fn handle(&mut self, _msg: User, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.users", &[]).unwrap();
        let results: Vec<User> = rows.iter()
            .map(User::from)
            .collect::<Vec<User>>();

        if results.is_empty() {
            Ok(User::default())
        } else {
            Ok(results[0].clone())
        }
    }
}

pub struct Users;

impl Message for Users {
    type Result = FieldResult<Vec<User>>;
}

impl Handler<Users> for Repo {
    type Result = FieldResult<Vec<User>>;

    fn handle(&mut self, _msg: Users, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.users", &[]).unwrap();
        let results: Vec<User> = rows.iter()
            .map(User::from)
            .collect::<Vec<User>>();

        Ok(results.clone())
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct UserClue {
    pub id: i32,
    pub user_id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nick_name: String,
    pub phone_number: String,
    pub mobile_number: String,
    pub gender: String,
    pub birth_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct UserWorkExperience {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub organization: String,
    pub location: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct UserWorkPreference {
    pub id: i32,
    pub user_id: i32,
    pub interests: i32,
    pub project_limit: i32,
    pub project_limit_updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
