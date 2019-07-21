use actix::prelude::*;
use juniper::{FieldError, FieldResult, RootNode};

use super::model::{Repo, Industry};

#[derive(Clone)]
pub struct Context {
    pub repo: Addr<Repo>,
}

impl juniper::Context for Context {}

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
        context
            .repo
            .send(Industry::with_id(1))
            .map_err(FieldError::from)
            .and_then(|res| res)
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
