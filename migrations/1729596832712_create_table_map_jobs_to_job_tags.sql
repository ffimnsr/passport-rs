-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS map_jobs_to_job_tags (
  job_id VARCHAR(24) NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
  job_tag_id BIGINT NOT NULL REFERENCES job_tags(id) ON DELETE CASCADE,
  PRIMARY KEY (job_id, job_tag_id)
);

-- !DOWN

DROP TABLE map_jobs_to_job_tags;
