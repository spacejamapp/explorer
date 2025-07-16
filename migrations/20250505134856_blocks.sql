-- Add migration script here
CREATE TABLE IF NOT EXISTS blocks (
  slot INT PRIMARY KEY,
  anchor INT NOT NULL,
  raw  TEXT NOT NULL
)
