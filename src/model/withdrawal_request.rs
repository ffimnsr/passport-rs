use std::convert::From;

use actix::prelude::*;
use chrono::NaiveDateTime;
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawalRequest {
    pub id: i32,
    pub requested_by_id: i32,
    pub approved_by_id: i32,
    pub bank_account_id: i32,
    pub amount: i32,
    pub reference_no: String,
    pub approved_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for WithdrawalRequest {
    fn default() -> Self {
        Self {
            id: 0,
            requested_by_id: 0,
            approved_by_id: 0,
            bank_account_id: 0,
            amount: 0,
            reference_no: "".to_owned(),
            approved_at: NaiveDateTime::from_timestamp(0, 0),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl WithdrawalRequest {
    #[allow(dead_code)]
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}

impl Message for WithdrawalRequest {
    type Result = FieldResult<WithdrawalRequest>;
}

impl From<Row> for WithdrawalRequest {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            requested_by_id: row.get(1),
            approved_by_id: row.get(2),
            bank_account_id: row.get(3),
            amount: row.get(4),
            reference_no: row.get(5),
            approved_at: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        }
    }
}

impl From<&Row> for WithdrawalRequest {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            requested_by_id: row.get(1),
            approved_by_id: row.get(2),
            bank_account_id: row.get(3),
            amount: row.get(4),
            reference_no: row.get(5),
            approved_at: row.get(6),
            created_at: row.get(7),
            updated_at: row.get(8),
        }
    }
}

impl Handler<WithdrawalRequest> for Repo {
    type Result = FieldResult<WithdrawalRequest>;

    fn handle()
}
