-- Your SQL goes here
CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);
CREATE TABLE reference_items (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  url TEXT
);
CREATE TABLE reference_authors (
  id SERIAL PRIMARY KEY,
  reference_id INTEGER NOT NULL,
  author_id INTEGER NOT NULL
)
