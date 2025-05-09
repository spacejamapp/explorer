-- Add migration script here
CREATE TABLE IF NOT EXISTS tickets (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  ticket_id VARCHAR NOT NULL UNIQUE,
  attempt SMALLINT NOT NULL
);
CREATE INDEX idx_tickets_block ON tickets (block);
