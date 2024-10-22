use actix_web::{
    delete,
    error::{self, ErrorBadRequest},
    get, patch, post, web, HttpRequest, HttpResponse, Result,
};
use log::debug;
use serde::Deserialize;
use sqlx::PgPool;
use fake::{Fake, Faker};

use crate::model::*;

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
pub struct PaginationQueryParams {
    l: Option<isize>,
    o: Option<isize>,
}

impl PaginationQueryParams {
    pub fn get_pagination(&self) -> PaginationParams {
        PaginationParams::new(self.l.unwrap_or(25).min(100), self.o.unwrap_or(0))
    }
}

#[derive(Debug, Deserialize)]
pub struct SimpleSearchQueryParams {
    query: String,
    l: Option<isize>,
    o: Option<isize>,
}

impl SimpleSearchQueryParams {
    pub fn get_pagination(&self) -> PaginationParams {
        PaginationParams::new(self.l.unwrap_or(25).min(100), self.o.unwrap_or(0))
    }
}

// Search for jobs
#[get("/search-jobs")]
pub async fn search_jobs(pool: web::Data<PgPool>, search: web::Query<SimpleSearchQueryParams>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Job::search(&mut pool, &search.query, Some(search.get_pagination()))
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "jobs": data, "count": data.len() })))
}

// Get all jobs list
#[get("/jobs-minimal")]
pub async fn get_all_jobs_minimal(pool: web::Data<PgPool>, paginate: web::Query<PaginationQueryParams>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Job::list(&mut pool, Some(paginate.get_pagination()))
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "jobs": data, "count": data.len() })))
}

// Get all jobs
#[get("/jobs")]
pub async fn get_all_jobs(pool: web::Data<PgPool>, paginate: web::Query<PaginationQueryParams>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Job::list(&mut pool, Some(paginate.get_pagination()))
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "jobs": data, "count": data.len() })))
}

// Get a single job by ID
#[get("/jobs/{id}")]
pub async fn get_job(pool: web::Data<PgPool>, id: web::Path<String>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Job::get_with_id(&mut pool, &id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

// Create a new job
#[post("/mgmt/jobs")]
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

// Update a job by ID
#[patch("/mgmt/jobs/{id}")]
pub async fn update_job(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
    job: web::Json<UpdateJob>,
) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    Job::update_with_id(&mut pool, &id, job.into_inner())
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

// Delete a job by ID
#[delete("/mgmt/jobs/{id}")]
pub async fn delete_job(pool: web::Data<PgPool>, id: web::Path<String>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    Job::delete_with_id(&mut pool, &id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

// Get all countries
#[get("/countries")]
pub async fn get_all_countries(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Country::list(&mut pool, None)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "countries": data, "count": data.len() })))
}

// Get all organizations
#[get("/orgs")]
pub async fn get_all_organizations(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Organization::list(&mut pool, Some(PaginationParams::new(25, 0)))
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "organizations": data, "count": data.len() })))
}

// Get a single organization by ID
#[get("/orgs/{id}")]
pub async fn get_organization(pool: web::Data<PgPool>, id: web::Path<i64>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = Organization::get_with_id(&mut pool, *id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

// Get all work functions
#[get("/work/functions")]
pub async fn get_all_work_functions(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = WorkFunction::list(&mut pool, None)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "work_functions": data, "count": data.len() })))
}

// Get all work industries
#[get("/work/industries")]
pub async fn get_all_work_industries(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let data = WorkIndustry::list(&mut pool, None)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "work_industries": data, "count": data.len() })))
}

// Create fake job data
#[get("/faker/create-job")]
pub async fn create_fake_job(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let job: NewJob = Faker.fake();
    let job_id = Job::insert(&mut pool, job)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "id": job_id })))
}

// Create a number of fake job data
#[get("/faker/create-job/{num}")]
pub async fn create_n_amount_of_fake_job(pool: web::Data<PgPool>, num: web::Path<i32>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;

    let mut jobs = vec![];

    for _i in 0..*num {
        let job: NewJob = Faker.fake();
        let job_id = Job::insert(&mut pool, job)
            .await
            .map_err(error::ErrorInternalServerError)?;
        jobs.push(job_id);
    }

    Ok(HttpResponse::Created().json(serde_json::json!({ "job_ids": jobs })))
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
