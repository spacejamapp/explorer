-- Add migration script here
CREATE TABLE IF NOT EXISTS validators (
  id SERIAL PRIMARY KEY,
  ed25519 VARCHAR NOT NULL UNIQUE,
  bandersnatch VARCHAR NOT NULL,
  name VARCHAR NOT NULL default '',
  details TEXT NOT NULL default '',
  software VARCHAR NOT NULL default '',
  ip VARCHAR NOT NULL default '',
  website VARCHAR NOT NULL default '',
  scores INT NOT NULL default 0
)
