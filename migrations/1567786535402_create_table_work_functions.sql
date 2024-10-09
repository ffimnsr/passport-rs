-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS work_functions (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE,
  status SMALLINT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

INSERT INTO work_functions
  (name, status, created_at, updated_at)
VALUES
  ('Engineering', 1, NOW(), NOW()),
  ('DevOps', 1, NOW(), NOW()),
  ('Accounting', 1, NOW(), NOW()),
  ('Legal', 1, NOW(), NOW()),
  ('Marketing', 1, NOW(), NOW()),
  ('Operations', 1, NOW(), NOW()),
  ('Designer', 1, NOW(), NOW()),
  ('Research', 1, NOW(), NOW()),
  ('Sales', 1, NOW(), NOW()),
  ('Support', 1, NOW(), NOW()),
  ('Virtual Assistant', 1, NOW(), NOW());

-- !DOWN

DROP TABLE work_functions;
