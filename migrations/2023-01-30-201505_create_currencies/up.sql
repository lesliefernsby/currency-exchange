-- Your SQL goes here
CREATE TABLE currencies (
  id SERIAL PRIMARY KEY,
  code VARCHAR NOT NULL,
  full_name TEXT NOT NULL,
  sign VARCHAR NOT NULL
)
