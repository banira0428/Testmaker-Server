-- Your SQL goes here
CREATE TABLE tests (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  color INTEGER NOT NULL DEFAULT 0,
  document_id VARCHAR NOT NULL,
  size INTEGER NOT NULL,
  comment text NOT NULL DEFAULT '',
  user_id VARCHAR NOT NULL,
  user_name VARCHAR NOT NULL
)