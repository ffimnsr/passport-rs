-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS user_leaderboard_ranks (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE,
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO user_leaderboard_ranks
  (name, status, created_at, updated_at)
VALUES
  ('Warrior', 1, NOW(), NOW()),
  ('Elite', 1, NOW(), NOW()),
  ('Master', 1, NOW(), NOW()),
  ('Grandmaster', 1, NOW(), NOW()),
  ('Epic', 1, NOW(), NOW()),
  ('Legend', 1, NOW(), NOW()),
  ('Mythic', 1, NOW(), NOW()),
  ('Mythical Glory', 1, NOW(), NOW()),
  ('Immortal', 1, NOW(), NOW());

-- !DOWN

DROP TABLE user_leaderboard_ranks;
