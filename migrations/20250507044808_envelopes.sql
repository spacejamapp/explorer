-- Add migration script here
CREATE TABLE IF NOT EXISTS envelopes (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  attempt SMALLINT NOT NULL,
  signature TEXT NOT NULL
);
CREATE INDEX idx_envelopes_block ON envelopes (block);
