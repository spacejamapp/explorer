-- Add migration script here
CREATE TABLE IF NOT EXISTS guarantees (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  report VARCHAR NOT NULL,
  slot INT NOT NULL,
  signatures VARCHAR[] NOT NULL
);
CREATE INDEX idx_guarantees_block ON guarantees (block);
