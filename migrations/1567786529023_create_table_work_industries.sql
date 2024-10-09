-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS work_industries (
  id SERIAL PRIMARY KEY,
  name TEXT,
  status SMALLINT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

-- !DOWN

DROP TABLE industries;
