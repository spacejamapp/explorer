-- Add migration script here
CREATE TABLE IF NOT EXISTS epoches (
  id SERIAL PRIMARY KEY,
  entropy VARCHAR NOT NULL UNIQUE,
  tickets_entropy VARCHAR NOT NULL,
  validators VARCHAR[] NOT NULL,
  validators_bandersnatches VARCHAR[] NOT NULL
)
