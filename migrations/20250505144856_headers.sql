-- Add migration script here
CREATE TABLE IF NOT EXISTS headers (
  slot INT PRIMARY KEY,
  hash VARCHAR NOT NULL,
  parent VARCHAR NOT NULL,
  parent_state_root VARCHAR NOT NULL,
  extrinsic_hash VARCHAR NOT NULL,
  extrinsic_count INT NOT NULL,
  author_id INT NOT NULL,
  entropy_source VARCHAR NOT NULL,
  seal VARCHAR NOT NULL,
  offenders_mark VARCHAR[] NOT NULL,
  current_epoch INT NOT NULL
)
