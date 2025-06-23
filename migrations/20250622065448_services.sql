-- Add migration script here
CREATE TABLE IF NOT EXISTS services (
  id INT NOT NULL UNIQUE,
  code VARCHAR NOT NULL,
  balance BIGINT NOT NULL,
  accumulate BIGINT NOT NULL,
  transfer BIGINT NOT NULL,
  total BIGINT NOT NULL,
  items INT NOT NULL
)
