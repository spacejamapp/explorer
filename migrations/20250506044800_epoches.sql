-- Add migration script here
CREATE TABLE IF NOT EXISTS epoches (
  id SERIAL PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  entropy VARCHAR NOT NULL, -- TODO: should be unique in production
  tickets_entropy VARCHAR NOT NULL,
  validators VARCHAR[] NOT NULL,
  validators_bandersnatches VARCHAR[] NOT NULL
);
CREATE INDEX idx_epoches_block ON epoches (block);
