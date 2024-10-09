-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS jobs (
  id SERIAL PRIMARY KEY,
  public_id VARCHAR(90) UNIQUE,
  title VARCHAR(300) NOT NULL,
  description TEXT NOT NULL,
  industry_id INTEGER REFERENCES industries(id) DEFAULT 1,
  country_id INTEGER REFERENCES countries(id) DEFAULT 1,
  organization_id INTEGER REFERENCES organizations(id) DEFAULT 1,
  work_function INTEGER REFERENCES work_functions(id) DEFAULT 1,
  work_experience_level INTEGER REFERENCES work_experience_levels(id) DEFAULT 1,
  work_contract_type INTEGER REFERENCES work_contract_types(id) DEFAULT 1,
  salary_upper_limit TEXT,
  salary_lower_limit TEXT,
  salary_currency VARCHAR(10),
  salary_timeframe SMALLINT,
  has_timetracker BOOLEAN DEFAULT FALSE,
  is_remote BOOLEAN DEFAULT TRUE,
  is_featured BOOLEAN DEFAULT FALSE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE jobs;
