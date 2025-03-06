-- Add up migration script here
CREATE TABLE IF NOT EXISTS CoreId (
    id UUID PRIMARY KEY,
    embedding_hash VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    breed VARCHAR NOT NULL,
    date_of_birth DATE NOT NULL,
    proof_level SMALLINT NOT NULL,
    microchip_id VARCHAR NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
