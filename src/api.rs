use actix_web::{error, web, HttpRequest, HttpResponse, Result};
use log::debug;
use sqlx::PgPool;

use crate::model::{job::NewJob, Job};

pub async fn version(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Open Sesame"))
}

pub async fn get_all_jobs(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut pool = pool.acquire().await.map_err(error::ErrorInternalServerError)?;
    let jobs = Job::all(&mut pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(jobs))
}

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
