use std::{env, io, sync::Arc};

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use futures::future::Future;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use log::info;

mod model;
mod schema;

use crate::schema::{create_schema, Database, Episode, Schema};

fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let ctx = Database(Episode::Jedi);

        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body(user))
    })
}

fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "passport=debug");
    env_logger::init();

    let addr = "127.0.0.1:8080";
    info!("Booting up server at {}", addr);

    let schema = Arc::new(create_schema());

    let app = move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    };

    HttpServer::new(app)
        .bind(addr)
        .expect("Failed to bind to port 8080")
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_get() {}
}
