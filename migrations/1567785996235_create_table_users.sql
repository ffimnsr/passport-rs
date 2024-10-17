-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL PRIMARY KEY,
  public_id VARCHAR(90) UNIQUE,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE users;
