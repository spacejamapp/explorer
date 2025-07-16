-- Add migration script here
CREATE TABLE IF NOT EXISTS epochs_validators (
  id SERIAL PRIMARY KEY,
  epoch INT NOT NULL,
  validator INT NOT NULL,
  vindex INT NOT NULL,
  blocks INT NOT NULL,
  tickets INT NOT NULL,
  preimages INT NOT NULL,
  guarantees INT NOT NULL,
  assurances INT NOT NULL
);
CREATE INDEX idx_validators_epoch ON epochs_validators (epoch);
CREATE INDEX idx_validators_validator ON epochs_validators (validator);
