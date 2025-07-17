-- Add migration script here
CREATE TABLE IF NOT EXISTS blocks (
  slot INT PRIMARY KEY,
  anchor_id INT NOT NULL,
  raw  TEXT NOT NULL
)
