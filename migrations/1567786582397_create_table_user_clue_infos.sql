-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS user_clue_infos (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  gender SMALLINT,
  birth_date DATE,
  tax_identification_no VARCHAR(60),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

-- !DOWN

DROP TABLE user_clues;
