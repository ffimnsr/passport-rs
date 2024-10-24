-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS organizations (
  id BIGSERIAL PRIMARY KEY,
  public_id TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  image_url TEXT NULL,
  website_url TEXT NULL,
  is_verified BOOL DEFAULT FALSE,
  is_featured BOOL DEFAULT FALSE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO organizations
  (public_id, name, description, is_verified, is_featured)
VALUES
  ('acme', 'Acme Corporation', 'The Acme Corporation is a fictional corporation that features prominently in the Road Runner/Wile E. Coyote animated shorts as a running gag. The company manufactures outlandish products that fail or backfire catastrophically at the worst possible times. The name is also used as a generic title in many cartoons, especially those made by Warner Bros. and films, TV series, commercials and comic strips.', TRUE, FALSE);

-- !DOWN

DROP TABLE organizations;
