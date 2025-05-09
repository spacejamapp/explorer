-- Add migration script here
CREATE TABLE IF NOT EXISTS preimages (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  requester INT NOT NULL,
  hash VARCHAR NOT NULL,
  blob BYTEA NOT NULL
);
CREATE INDEX idx_preimages_block ON preimages (block);
