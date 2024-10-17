-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS organizations (
  id BIGSERIAL PRIMARY KEY,
  public_id TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  is_verified BOOL DEFAULT FALSE,
  is_featured BOOL DEFAULT FALSE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO organizations
  (public_id, name, is_verified, is_featured)
VALUES
  ('acme', 'Acme Corporation', TRUE, FALSE);

-- !DOWN

DROP TABLE organizations;
