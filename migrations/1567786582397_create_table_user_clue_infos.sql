-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS user_clue_infos (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
  gender SMALLINT,
  birth_date DATE,
  tax_identification_no VARCHAR(60),
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE user_clue_infos;
