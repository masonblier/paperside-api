-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  doublehashed TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  hashed_access_token TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  last_accessed_at TIMESTAMPTZ NOT NULL,
  accessed_by_client_ip TEXT
)
