-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL PRIMARY KEY,
  public_id VARCHAR(90) UNIQUE,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

-- !DOWN

DROP TABLE users;
