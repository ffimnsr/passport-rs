-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS job_tags (
  id BIGSERIAL PRIMARY KEY,
  name TEXT UNIQUE,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE job_tags;
