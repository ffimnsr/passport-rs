-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS jobs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(300) NOT NULL UNIQUE,
  description TEXT NOT NULL,
  experience_level SMALLINT NOT NULL DEFAULT 1,
  salary_upper_limit TEXT,
  salary_lower_limit TEXT,
  salary_currency VARCHAR(10),
  salary_timeframe SMALLINT,
  work_type SMALLINT NOT NULL DEFAULT 1,
  has_timetracker BOOLEAN DEFAULT FALSE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE jobs;
