-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS organizations (
  id SERIAL PRIMARY KEY,
  public_id VARCHAR(90) UNIQUE,
  name TEXT NOT NULL UNIQUE,
  is_verified BOOLEAN DEFAULT FALSE,
  is_featured BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

-- !DOWN

DROP TABLE organizations;
