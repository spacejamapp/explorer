-- Add migration script here
CREATE TABLE IF NOT EXISTS cores (
  id SERIAL PRIMARY KEY,
  epoch INT NOT NULL,
  vindex INT NOT NULL,
  gas_used BIGINT NOT NULL,
  imports INT NOT NULL,
  extrinsic_count INT NOT NULL,
  extrinsic_size INT NOT NULL,
  exports INT NOT NULL,
  bundle_size INT NOT NULL,
  da_load BIGINT NOT NULL,
  popularity BIGINT NOT NULL
);
CREATE INDEX idx_cores_epoch ON cores (epoch);
CREATE INDEX idx_cores_vindex ON cores (vindex);
