-- Add migration script here
CREATE TABLE IF NOT EXISTS guarantees (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  slot INT NOT NULL,
  signatures VARCHAR[] NOT NULL,
  spec VARCHAR NOT NULL,
  core INT NOT NULL,
  authorizer_hash VARCHAR NOT NULL,
  auth_output TEXT NOT NULL,
  auth_gas BIGINT NOT NULL
);
CREATE INDEX idx_guarantees_block ON guarantees (block);
