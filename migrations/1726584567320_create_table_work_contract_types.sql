-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS work_contract_type (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE,
  status SMALLINT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

INSERT INTO work_contract_type
  (name, status, created_at, updated_at)
VALUES
  ('Full-Time', 1, NOW(), NOW()),
  ('Part-Time', 1, NOW(), NOW()),
  ('Freelance Contract', 1, NOW(), NOW()),
  ('Fixed-Term Contract', 1, NOW(), NOW()),
  ('Zero-Hour Contract', 1, NOW(), NOW()),
  ('Internship', 1, NOW(), NOW());

-- !DOWN

DROP TABLE organizations;
