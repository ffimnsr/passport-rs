use std::convert::From;

use actix::prelude::*;
use chrono::NaiveDateTime;
use juniper::{FieldResult, GraphQLObject};
use postgres::row::Row;
use serde::{Deserialize, Serialize};

use super::{Connection, Repo};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct OauthIdentity {
    pub id: i32,
    pub user_id: i32,
    pub account_email: String,
    pub account_id: String,
    pub provider: String,
    pub public_profile_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for OauthIdentity {
    fn default() -> Self {
        Self {
            id: 0,
            user_id: 0,
            account_email: String::new(),
            account_id: String::new(),
            provider: String::new(),
            public_profile_url: String::new(),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl OauthIdentity {
    #[allow(dead_code)]
    pub fn with_id(id: i32) -> Self {
        let mut model = Self::default();
        model.id = id;
        model
    }
}
