-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  passhash VARCHAR(122) NOT NULL, --argon hash
  created_at TIMESTAMPTZ NOT NULL
);
CREATE TABLE sessions (
  token VARCHAR(32) NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  last_accessed_at TIMESTAMPTZ NOT NULL,
  accessed_by_client_ip TEXT
)
