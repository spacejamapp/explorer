-- Add migration script here
CREATE TABLE IF NOT EXISTS tickets (
  id SERIAL PRIMARY KEY,
  ticket_id VARCHAR NOT NULL UNIQUE,
  attempt SMALLINT NOT NULL
)
