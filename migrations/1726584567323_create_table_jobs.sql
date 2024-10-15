-- # Put the your SQL below migration seperator.
-- !UP

CREATE TYPE work_experience_level AS ENUM (
  'intern',
  'entry_level',
  'mid_level',
  'senior_level',
  'executive'
);

CREATE TYPE work_contract_type AS ENUM (
  'full_time',
  'part_time',
  'freelance_contract',
  'fixed_term_contract',
  'zero_hour_contract',
  'internship'
);

CREATE TYPE salary_timeframe AS ENUM (
  'hourly',
  'daily',
  'weekly',
  'semi_monthly',
  'monthly',
  'quarterly',
  'annually'
);

CREATE TYPE salary_detail AS (
  upper_limit TEXT,
  lower_limit TEXT,
  currency VARCHAR(10),
  timeframe salary_timeframe
);

CREATE TABLE IF NOT EXISTS jobs (
  id VARCHAR(24) PRIMARY KEY,
  title VARCHAR(300) NOT NULL,
  description TEXT NOT NULL,
  industry_id INT REFERENCES work_industries(id) DEFAULT 1000,
  country_id INT REFERENCES countries(id) DEFAULT 1,
  organization_id BIGINT REFERENCES organizations(id) DEFAULT 1,
  work_experience_level work_experience_level DEFAULT 'intern',
  work_contract_type work_contract_type DEFAULT 'full_time',
  salary salary_detail,
  has_timetracker BOOL DEFAULT FALSE,
  is_remote BOOL DEFAULT TRUE,
  is_featured BOOL DEFAULT FALSE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE jobs;
DROP TYPE salary_detail;
DROP TYPE salary_timeframe;
DROP TYPE work_contract_type;
DROP TYPE work_experience_level;
