use actix::prelude::*;
use juniper::{FieldResult, RootNode};
use super::model::Repo;

#[derive(Clone)]
pub struct Context {
    pub repo: Addr<Repo>,
}

impl juniper::Context for Context {}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A structure that defines project industry")]
struct Industry {
    id: i32,
    name: String,
    status: i32,
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
        Ok(Industry {
            id: 1234,
            name: "Sample".to_owned(),
            status: 0,
        })
    }

    #[graphql(arguments(id(description = "The id of the industry")))]
    fn industry(context: &Context, id: i32) -> FieldResult<Industry> {
        Ok(Industry {
            id: 1234,
            name: "Sample".to_owned(),
            status: 0,
        })
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn createIndustry(context: &Context, new_industry: NewIndustry) -> FieldResult<Industry> {
        Ok(Industry {
            id: 1234,
            name: new_industry.name,
            status: new_industry.status,
        })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
