use juniper::{Context, FieldResult, RootNode};

#[derive(juniper::GraphQLEnum, Clone, Copy)]
pub enum Episode {
    Empire,
    Jedi,
}

pub struct Database(pub Episode);

impl Context for Database {}

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

#[juniper::object(Context = Database)]
impl Query {
    fn industries(context: &Database) -> FieldResult<Industry> {
        Ok(Industry {
            id: 1234,
            name: "Sample".to_owned(),
            status: 0,
        })
    }

    #[graphql(arguments(id(description = "The id of the industry")))]
    fn industry(context: &Database, id: i32) -> FieldResult<Industry> {
        Ok(Industry {
            id: 1234,
            name: "Sample".to_owned(),
            status: 0,
        })
    }
}

pub struct Mutation;

#[juniper::object(Context = Database)]
impl Mutation {
    fn createIndustry(context: &Database, new_industry: NewIndustry) -> FieldResult<Industry> {
        Ok(Industry {
            id: 1234,
            name: new_industry.name,
            status: new_industry.status,
        })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
