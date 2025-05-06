-- Add migration script here
CREATE TABLE IF NOT EXISTS headers (
    id SERIAL PRIMARY KEY,
    hash VARCHAR NOT NULL,
    parent VARCHAR NOT NULL,
    parent_state_root VARCHAR NOT NULL,
    extrinsic_hash VARCHAR NOT NULL,
    slot INT NOT NULL,
    epoch_mark VARCHAR,
    tickets_mark VARCHAR[],
    offenders_mark VARCHAR[] NOT NULL,
    author_index INT NOT NULL,
    entropy_source VARCHAR NOT NULL,
    seal VARCHAR NOT NULL
)
