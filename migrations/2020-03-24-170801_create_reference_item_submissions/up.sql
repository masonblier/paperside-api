-- Your SQL goes here
CREATE TABLE reference_item_submissions (
  id SERIAL PRIMARY KEY,
  reference_item_id INTEGER NOT NULL,
  submitting_user_id INTEGER NOT NULL,
  is_public BOOLEAN NOT NULL
);
CREATE INDEX idx_reference_item_submissions ON reference_item_submissions(reference_item_id,submitting_user_id);