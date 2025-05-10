-- Add migration script here
CREATE TABLE IF NOT EXISTS epoches (
  id INT PRIMARY KEY,
  block INT NOT NULL REFERENCES blocks(slot),
  entropy VARCHAR NOT NULL, -- TODO: should be unique in production
  tickets_entropy VARCHAR NOT NULL,
  validators VARCHAR[] NOT NULL,
  validators_bandersnatches VARCHAR[] NOT NULL,
  blocks INT NOT NULL DEFAULT 0,
  tickets INT NOT NULL DEFAULT 0,
  preimages INT NOT NULL DEFAULT 0,
  preimages_size INT NOT NULL DEFAULT 0,
  guarantees INT NOT NULL DEFAULT 0,
  assurances INT NOT NULL DEFAULT 0
);
CREATE INDEX idx_epoches_block ON epoches (block);
