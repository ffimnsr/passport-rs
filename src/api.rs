use actix_web::{web, error, HttpRequest, HttpResponse, Result};
use futures::{FutureExt, TryFutureExt};
use sqlx::{AnyPool, Database, Pool, SqlitePool};

use crate::model::{job::NewJob, Job};

pub async fn version(_req: HttpRequest) -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().body("Open Sesame"))
}

pub async fn get_all_jobs(
  pool: web::Data<AnyPool>,
) -> Result<HttpResponse> {
  let mut pool = pool.acquire().await
    .map_err(error::ErrorInternalServerError)?;
  let jobs = Job::all(&mut pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(jobs))
}

pub async fn create_job(
  pool: web::Data<AnyPool>,
  job: web::Json<NewJob>,
) -> Result<HttpResponse> {
  let mut pool = pool.acquire().await
    .map_err(error::ErrorInternalServerError)?;
  let job = job.into_inner();
  let job_id = Job::insert(job, &mut pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(job_id))
}
