-- Add migration script here
CREATE TABLE IF NOT EXISTS blocks (
  slot INT PRIMARY KEY,
  raw  TEXT NOT NULL
)
