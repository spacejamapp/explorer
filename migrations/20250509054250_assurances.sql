-- Add migration script here
CREATE TABLE IF NOT EXISTS assurances (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  anchor VARCHAR NOT NULL,
  bitfield VARCHAR NOT NULL,
  validator_index INT NOT NULL,
  signature VARCHAR NOT NULL
);
CREATE INDEX idx_assurances_block ON assurances (block);
