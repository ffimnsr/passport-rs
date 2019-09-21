use std::convert::From;

use actix::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct BankAccount {
    pub id: i32,
    pub user_id: i32,
    pub account_name: String,
    pub account_no: String,
    pub bank_address: String,
    pub bank_branch: String,
    pub swift_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for BankAccount {
    fn default() -> Self {
        Self {
            id: 0,
            user_id: 0,
            account_name: String::new(),
            account_no: String::new(),
            bank_address: String::new(),
            bank_branch: String::new(),
            swift_code: String::new(),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl BankAccount {
    #[allow(dead_code)]
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for BankAccount {
    type Result = FieldResult<BankAccount>;
}

impl From<Row> for BankAccount {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            user_id: row.get(1),
            account_name: row.get(2),
            account_no: row.get(3),
            bank_address: row.get(4),
            bank_branch: row.get(5),
            swift_code: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        }
    }
}

impl From<&Row> for BankAccount {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            user_id: row.get(1),
            account_name: row.get(2),
            account_no: row.get(3),
            bank_address: row.get(4),
            bank_branch: row.get(5),
            swift_code: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        }
    }
}

impl Handler<BankAccount> for Repo {
    type Result = FieldResult<BankAccount>;

    fn handle(&mut self, _msg: BankAccount, _ctx: &mut Self::Context) -> Self::Result {
        let client: &mut Connection = &mut self.0.get().unwrap();
        let rows: Vec<Row> = client.query("SELECT * FROM public.bank_accounts", &[]).unwrap();
        let results: Vec<BankAccount> = rows.iter()
            .map(BankAccount::from)
            .collect::<Vec<BankAccount>>();

        if results.is_empty() {
            Ok(BankAccount::default())
        } else {
            Ok(results[0].clone())
        }
    }
}
