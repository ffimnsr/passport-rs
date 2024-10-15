-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS user_clue_work_preferences (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT REFERENCES users(id),
  interests SMALLINT[],
  project_limit SMALLINT,
  project_limit_updated_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

-- !DOWN

DROP TABLE user_clue_work_preferences;
