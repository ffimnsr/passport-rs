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
  (id, name, status, created_at, updated_at)
VALUES
  (1000, 'Engineering', 1, NOW(), NOW()),
  (1001, 'DevOps', 1, NOW(), NOW()),
  (1002, 'Accounting', 1, NOW(), NOW()),
  (1003, 'Legal', 1, NOW(), NOW()),
  (1004, 'Marketing', 1, NOW(), NOW()),
  (1005, 'Operations', 1, NOW(), NOW()),
  (1006, 'Designer', 1, NOW(), NOW()),
  (1007, 'Research', 1, NOW(), NOW()),
  (1008, 'Sales', 1, NOW(), NOW()),
  (1009, 'Support', 1, NOW(), NOW()),
  (1010, 'Virtual Assistant', 1, NOW(), NOW());

-- !DOWN

DROP TABLE work_functions;
