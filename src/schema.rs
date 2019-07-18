use actix::prelude::*;
use juniper::{FieldResult, FieldError, RootNode};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use super::model::Repo;

#[derive(Clone)]
pub struct Context {
    pub repo: Addr<Repo>,
}

impl juniper::Context for Context {}

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug)]
#[graphql(description = "A structure that defines project industry")]
pub struct Industry {
    id: i32,
    name: String,
    status: i32,
    inserted_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Industry {
    pub fn with_id(id: i32) -> Industry {
        let mut model = Industry::default();
        model.id = id;
        model
    }
}

impl Default for Industry {
    fn default() -> Industry {
        Industry {
            id: 0,
            name: "".to_owned(),
            status: 0,
            inserted_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

// impl<A, M> MessageResponse<A, M> for Industry
// where
//     A: Actor,
//     M: Message<Result = Industry>,
// {
//     fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
//         if let Some(tx) = tx {
//             tx.send(self)
//         }
//     }
// }

impl Message for Industry {
    type Result = FieldResult<Industry>;
}

impl Handler<Industry> for Repo {
    type Result = FieldResult<Industry>;

    fn handle(&mut self, _msg: Industry, _ctx: &mut Self::Context) -> Self::Result {
        Ok(Industry::with_id(3))
    }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A structure that defines project industry")]
struct NewIndustry {
    name: String,
    status: i32,
}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    fn industries(context: &Context) -> FieldResult<Industry> {        
        Ok(Industry::default())
    }

    #[graphql(arguments(id(description = "The id of the industry")))]
    fn industry(context: &Context, id: i32) -> FieldResult<Industry> {
        context.repo
            .send(Industry::with_id(1))
            .map_err(FieldError::from)
            .and_then(|res| { res })
            .wait()
    }
}


pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn createIndustry(context: &Context, new_industry: NewIndustry) -> FieldResult<Industry> {
        Ok(Industry {
            name: new_industry.name,
            status: new_industry.status,
            ..Industry::default()
        })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
