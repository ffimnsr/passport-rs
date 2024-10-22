-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS user_clue_work_preferences (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
  interests SMALLINT[],
  project_limit SMALLINT DEFAULT 5,
  project_limit_updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- !DOWN

DROP TABLE user_clue_work_preferences;
