-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS work_experience_levels (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE,
  status SMALLINT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

INSERT INTO work_experience_levels
  (name, status, created_at, updated_at)
VALUES
  ('Intern', 1, NOW(), NOW()),
  ('Entry Level', 1, NOW(), NOW()),
  ('Mid Level', 1, NOW(), NOW()),
  ('Senior Level', 1, NOW(), NOW()),
  ('Executive', 1, NOW(), NOW());

-- !DOWN

DROP TABLE organizations;
