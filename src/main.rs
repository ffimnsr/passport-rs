use std::{env};

use actix_web::{web, middleware, App, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::{info};

mod models;

use self::models::industry::NewIndustry;

fn index(pl: web::Json<NewIndustry>, req: HttpRequest) -> HttpResponse {
    info!("request: {:?}", req);
    info!("model: {:?}", pl);
    HttpResponse::Ok().json(pl.0)
}

fn main() {
    dotenv().ok();

    env::set_var("RUST_LOG", "passport=debug");
    env_logger::init();


    let addr = "127.0.0.1:8080";
    info!("Booting up server at {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::post().to(index)))
    })
        .bind(addr)
        .expect("Cannot bind to port 8080")
        .run()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_get() {

    }
}
