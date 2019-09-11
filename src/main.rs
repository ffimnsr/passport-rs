use std::str::FromStr;
use std::sync::Arc;
use std::{env, io, net};

#[cfg(target_family = "unix")]
use std::os::unix::io::FromRawFd;

use actix::prelude::*;
use actix_web::http::StatusCode;
use actix_web::{guard, middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use futures::future::Future;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use log::info;
use postgres::{config::Config, NoTls};
use r2d2_postgres::PostgresConnectionManager;

mod model;
mod schema;

use crate::model::Repo;
use crate::schema::{create_schema, Context, Schema};

struct AppState {
    executor: GraphQLExecutor,
}

#[derive(Clone)]
struct GraphQLExecutor {
    schema: Arc<Schema>,
    context: Context,
}

impl GraphQLExecutor {
    fn new(schema: Arc<Schema>, context: Context) -> GraphQLExecutor {
        GraphQLExecutor { schema, context }
    }
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st.executor.schema, &st.executor.context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|out| {
        Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body(out))
    })
}

fn error_404() -> HttpResponse {
    let data = serde_json::json!({
        "message": "404 Page Not Found",
    });

    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("application/json; charset=utf-8")
        .json(data)
}

fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "passport=debug");
    env_logger::init();

    let sys = actix::System::new("passport");

    let config_url =
        env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/passport".to_owned());

    let config = Config::from_str(&config_url).unwrap();
    let manager = PostgresConnectionManager::new(config, NoTls);

    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let fd = env::var("LISTEN_FD")
        .ok()
        .and_then(|d| d.parse().ok())
        .expect("No provided FD");

    info!("Booting up server at FD {}", fd);

    let listener: net::TcpListener;
    unsafe {
        listener = net::TcpListener::from_raw_fd(fd);
    }

    let repo_addr = SyncArbiter::start(3, move || Repo(pool.clone()));

    let schema_context = Context {
        repo: repo_addr.clone(),
    };
    let schema = Arc::new(create_schema());

    let app = move || {
        App::new()
            .data(AppState {
                executor: GraphQLExecutor::new(schema.clone(), schema_context.clone()),
            })
            .data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .default_service(
                web::resource("").route(web::get().to(error_404)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
            )
    };

    HttpServer::new(app)
        .listen(listener)
        .expect("Failed to bind FD")
        .start();

    sys.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[test]
    fn test_error_404_ok() {
        let res = test::block_on(error_404()).unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_graphiql_ok() {
        let res = test::block_on(graphiql()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    fn test_graphiql_not_ok() {
        let res = test::block_on(graphiql()).unwrap();
        assert_ne!(res.status(), StatusCode::BAD_REQUEST);
    }
}
