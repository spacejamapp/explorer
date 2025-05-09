-- Add migration script here
CREATE TABLE IF NOT EXISTS dispute_verdicts (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  target VARCHAR NOT NULL,
  age INT NOT NULL,
  votes VARCHAR[] NOT NULL
);
CREATE INDEX idx_dispute_verdicts_block ON dispute_verdicts (block);

CREATE TABLE IF NOT EXISTS dispute_culprits (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  target VARCHAR NOT NULL,
  key VARCHAR NOT NULL,
  signature VARCHAR NOT NULL
);
CREATE INDEX idx_dispute_culprits_block ON dispute_culprits (block);

CREATE TABLE IF NOT EXISTS dispute_faults (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  target VARCHAR NOT NULL,
  vote BOOLEAN NOT NULL,
  key VARCHAR NOT NULL,
  signature VARCHAR NOT NULL
);
CREATE INDEX idx_dispute_faults_block ON dispute_faults (block);
