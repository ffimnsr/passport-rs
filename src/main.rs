use std::{env, io};

use actix_cors::Cors;
use actix_web::http::{header, Method};
use actix_web::{get, guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use listenfd::ListenFd;

mod api;
mod db;
mod model;
mod service_error;
mod utils;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let payload = serde_json::json!({
        "message": format!("Open Sesame {PKG_VERSION}")
    });
    Ok(HttpResponse::Ok().json(payload))
}

async fn default_handler(_req_method: Method) -> Result<impl Responder> {
    Ok(HttpResponse::MethodNotAllowed().finish())
}

fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    _req: &HttpRequest,
) -> actix_web::error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        },
        _ => HttpResponse::BadRequest().body(detail),
    };

    actix_web::error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).await.expect("Failed to create pool");

    let app = move || {
        let json_cfg = web::JsonConfig::default()
            .error_handler(json_error_handler)
            .limit(4096);
        let pool = web::Data::new(pool.clone());
        App::new()
            .app_data(pool)
            .app_data(json_cfg)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8000")
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().ends_with(b".sesame-landing-ifn4.pages.dev")
                            || origin.as_bytes().ends_with(b".se-same.com")
                    })
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(index)
            .service(web::resource("/search-jobs").route(web::get().to(api::search_jobs)))
            .service(web::resource("/jobs-minimal").route(web::get().to(api::get_all_jobs_minimal)))
            .service(web::resource("/jobs").route(web::get().to(api::get_all_jobs)))
            .service(web::resource("/jobs/{id}").route(web::get().to(api::get_job)))
            .service(web::resource("/countries").route(web::get().to(api::get_all_countries)))
            .service(web::resource("/orgs").route(web::get().to(api::get_all_organizations)))
            .service(web::resource("/orgs/{id}").route(web::get().to(api::get_organization)))
            .service(web::resource("/work/functions").route(web::get().to(api::get_all_work_functions)))
            .service(web::resource("/work/industries").route(web::get().to(api::get_all_work_industries)))
            .service(web::resource("/mgmt/jobs").route(web::post().to(api::create_job)))
            .service(web::resource("/mgmt/jobs/{id}").route(web::delete().to(api::delete_job)))
            .service(api::create_fake_job)
            .service(web::resource("/version").guard(guard::Get()).to(api::version))
            .service(web::resource("/health").guard(guard::Get()).to(api::health_check))
            .default_service(web::to(default_handler))
    };

    let is_dev = env::var("DEV_MODE").is_ok();
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    let server = if is_dev {
        let mut listenfd = ListenFd::from_env();
        if let Some(lstn) = listenfd.take_tcp_listener(0).expect("Failed to take FD") {
            HttpServer::new(app).listen(lstn).expect("Failed to bind FD")
        } else {
            HttpServer::new(app)
                .bind(("127.0.0.1", port))
                .expect("Failed to bind local port")
        }
    } else {
        HttpServer::new(app)
            .bind(("0.0.0.0", port))
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
        let expected_body = serde_json::json!({
            "message": format!("Open Sesame {PKG_VERSION}")
        });
        assert_eq!(
            to_bytes(response_body).await?,
            expected_body.to_string().as_bytes()
        );
        Ok(())
    }
}
