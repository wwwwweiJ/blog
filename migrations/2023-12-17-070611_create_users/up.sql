-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY NOT NULL,
  first_name VARCHAR,
  last_name VARCHAR,
  password Text NOT NULL,
  email VARCHAR NOT NULL
)
