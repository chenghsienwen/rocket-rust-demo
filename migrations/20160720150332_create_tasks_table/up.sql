CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    game_id SERIAL DEFAULT 1,
    name TEXT NOT NULL,
    score  SERIAL DEFAULT 0,
    ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  status TEXT NOT NULL,
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('games');
