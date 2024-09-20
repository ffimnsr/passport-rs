use std::{env, io};

use actix_web::http::Method;
use actix_web::{get, guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use listenfd::ListenFd;

mod api;
mod db;
mod model;
// mod schema;

// use crate::model::Repo;
// use crate::schema::{create_schema, Context, Schema};

// struct AppState {
//     executor: GraphQLExecutor,
// }

// #[derive(Clone)]
// struct GraphQLExecutor {
//     schema: Arc<Schema>,
//     context: Context,
// }

// impl GraphQLExecutor {
//     fn new(schema: Arc<Schema>, context: Context) -> GraphQLExecutor {
//         GraphQLExecutor { schema, context }
//     }
// }

// fn graphiql() -> HttpResponse {
//     let html = graphiql_source("/graphql");
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html)
// }

// fn graphql(
//     st: web::Data<AppState>,
//     data: web::Json<GraphQLRequest>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     web::block(move || {
//         let res = data.execute(&st.executor.schema, &st.executor.context);
//         Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
//     })
//     .map_err(Error::from)
//     .and_then(|out| {
//         Ok(HttpResponse::Ok()
//             .content_type("application/json; charset=utf-8")
//             .body(out))
//     })
// }

#[get("/")]
async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let payload = serde_json::json!({
        "message": "Open Sesame"
    });
    Ok(HttpResponse::Ok().json(payload))
}

async fn default_handler(_req_method: Method) -> Result<impl Responder> {
    // match req_method {
    //     _ => Ok(HttpResponse::MethodNotAllowed().finish()),
    // }
    Ok(HttpResponse::MethodNotAllowed().finish())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(&database_url).await.expect("Failed to create pool");

    let app = move || {
        let json_cfg = web::JsonConfig::default().limit(4096);
        let pool = web::Data::new(pool.clone());
        App::new()
            .app_data(pool)
            .app_data(json_cfg)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(
                web::resource("/jobs")
                    .route(web::get().to(api::get_all_jobs))
                    .route(web::post().to(api::create_job)),
            )
            .service(web::resource("/version").guard(guard::Get()).to(api::version))
            .default_service(web::to(default_handler))
    };

    let mut listenfd = ListenFd::from_env();
    let server = if let Some(lstn) = listenfd.take_tcp_listener(0).expect("Failed to take FD") {
        HttpServer::new(app).listen(lstn).expect("Failed to bind FD")
    } else {
        HttpServer::new(app)
            .bind(("127.0.0.1", 8080))
            .expect("Failed to bind local port")
    };

    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, dev::Service, http, test, Error};

    #[test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().service(index);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Open Sesame"##);
        Ok(())
    }
}
