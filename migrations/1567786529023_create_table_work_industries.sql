-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS work_industries (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE,
  status SMALLINT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

INSERT INTO work_industries
  (id, name, status, created_at, updated_at)
VALUES
  (1000, 'Administration and support', 1, NOW(), NOW()),
  (1001, 'Architecture and engineering', 1, NOW(), NOW()),
  (1002, 'Art and design', 1, NOW(), NOW()),
  (1003, 'Business and finance operations', 1, NOW(), NOW()),
  (1004, 'Community and social services', 1, NOW(), NOW()),
  (1005, 'Computer and technology', 1, NOW(), NOW()),
  (1006, 'Education', 1, NOW(), NOW()),
  (1007, 'Legal', 1, NOW(), NOW()),
  (1008, 'Marketing', 1, NOW(), NOW()),
  (1009, 'Sales', 1, NOW(), NOW()),
  (1010, 'Other professional services', 1, NOW(), NOW());

-- !DOWN

DROP TABLE work_industries;
