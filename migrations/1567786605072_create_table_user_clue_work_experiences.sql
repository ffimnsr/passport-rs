-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS user_work_experiences (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  title TEXT,
  organization VARCHAR(200),
  location TEXT,
  from_date DATE,
  to_date DATE,
  description TEXT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

-- !DOWN

DROP TABLE user_work_experiences;
