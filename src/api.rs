use actix_web::{
    error::{self, ErrorBadRequest},
    get, web, HttpRequest, HttpResponse, Result,
};
use log::debug;
use serde::Deserialize;
use sqlx::PgPool;

use crate::model::{job::NewJob, Job, PaginationParams};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

// Get the version of the API
pub async fn version(_req: HttpRequest) -> Result<HttpResponse> {
    let payload = serde_json::json!({
        "message": format!("Open Sesame {PKG_VERSION}")
    });
    Ok(HttpResponse::Ok().json(payload))
}

// Health check endpoint
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Deserialize)]
pub struct SimpleSearch {
    query: String,
}

// Search for jobs
pub async fn search_jobs(pool: web::Data<PgPool>, search: web::Query<SimpleSearch>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let jobs = Job::search(&mut pool, &search.query, None)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(jobs))
}

// Get all jobs list
pub async fn get_all_jobs_minimal(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let limit = 25;
    let jobs = Job::list(&mut pool, Some(PaginationParams::new(limit, 0)))
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(jobs))
}

// Get all jobs
pub async fn get_all_jobs(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let limit = 25;
    let jobs = Job::list(&mut pool, Some(PaginationParams::new(limit, 0)))
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "jobs": jobs, "count": limit })))
}

// Get a single job by ID
pub async fn get_job(pool: web::Data<PgPool>, id: web::Path<String>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let job = Job::get_with_id(&mut pool, &id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(job))
}

// Create a new job
pub async fn create_job(pool: web::Data<PgPool>, job: web::Json<NewJob>) -> Result<HttpResponse> {
    use validator::Validate;

    let mut pool = pool
        .acquire()
        .await
        .inspect_err(|e| debug!("Error acquiring pool: {:?}", e))
        .map_err(error::ErrorInternalServerError)?;

    debug!("Creating job: {:?}", job);
    let job = job.into_inner();

    // Validate the job data
    job.validate().map_err(ErrorBadRequest)?;

    let job_id = Job::insert(&mut pool, job)
        .await
        .inspect_err(|e| debug!("Error inserting job: {:?}", e))
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "id": job_id })))
}

// Create fake job data
#[get("/faker/create-job")]
pub async fn create_fake_job(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    use fake::faker::company::en::Profession;
    use fake::faker::lorem::en::Sentence;
    use fake::Fake;

    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let job = NewJob {
        title: Profession().fake(),
        description: Sentence(1..30).fake(),
    };

    let job_id = Job::insert(&mut pool, job)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "id": job_id })))
}

// Delete a job by ID
pub async fn delete_job(pool: web::Data<PgPool>, id: web::Path<String>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    Job::delete_with_id(&mut pool, &id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, http, test, App, Error};

    // Unit test for the version function
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

    // Integration test for the version function
    #[test]
    async fn test_version_integration() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(version));
        let app = test::init_service(app).await;

        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

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
