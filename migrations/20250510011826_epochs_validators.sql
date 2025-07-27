-- Add migration script here
CREATE TABLE IF NOT EXISTS epochs_validators (
  id SERIAL PRIMARY KEY,
  epoch_id INT NOT NULL,
  validator_id INT NOT NULL,
  vindex INT NOT NULL,
  blocks INT NOT NULL,
  tickets INT NOT NULL,
  preimages INT NOT NULL,
  guarantees INT NOT NULL,
  assurances INT NOT NULL
);
CREATE INDEX idx_validators_epoch ON epochs_validators (epoch_id);
CREATE INDEX idx_validators_validator ON epochs_validators (validator_id);
