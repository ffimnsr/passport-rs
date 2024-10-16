-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS organizations (
  id BIGSERIAL PRIMARY KEY,
  public_id TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  is_verified BOOL DEFAULT FALSE,
  is_featured BOOL DEFAULT FALSE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

INSERT INTO organizations
  (public_id, name, is_verified, is_featured, created_at, updated_at)
VALUES
  ('acme', 'Acme Corporation', TRUE, FALSE, NOW(), NOW());

-- !DOWN

DROP TABLE organizations;
