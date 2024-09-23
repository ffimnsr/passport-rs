use actix_web::{error, web, HttpRequest, HttpResponse, Result};
use log::debug;
use sqlx::PgPool;

use crate::model::{job::NewJob, Job};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

// Get the version of the API
pub async fn version(_req: HttpRequest) -> Result<HttpResponse> {
    let payload = serde_json::json!({
        "message": format!("Open Sesame {PKG_VERSION}")
    });
    Ok(HttpResponse::Ok().json(payload))
}

// Get all jobs
pub async fn get_all_jobs(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let jobs = Job::all(&mut pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(jobs))
}

// Get a single job by ID
pub async fn get_job(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let job = Job::get_with_id(id.into_inner(), &mut pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(job))
}

// Create a new job
pub async fn create_job(pool: web::Data<PgPool>, job: web::Json<NewJob>) -> Result<HttpResponse> {
    let mut pool = pool
        .acquire()
        .await
        .inspect_err(|e| debug!("Error acquiring pool: {:?}", e))
        .map_err(error::ErrorInternalServerError)?;

    debug!("Creating job: {:?}", job);
    let job = job.into_inner();
    let job_id = Job::insert(job, &mut pool)
        .await
        .inspect_err(|e| debug!("Error inserting job: {:?}", e))
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "id": job_id })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, dev::Service, http, test, App, Error};

    #[test]
    async fn test_version() -> Result<(), Error> {
        let req = test::TestRequest::default().to_http_request();

        let resp = version(req).await?;
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

    #[test]
    async fn test_version_integration() -> Result<(), Error> {
        let app = App::new().route("/version", web::get().to(version));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/version").to_request();
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
