-- Add up migration script here
CREATE TABLE IF NOT EXISTS MerchantJoinIdTree (
    storage_id SERIAL PRIMARY KEY,
    leaves JSONB NOT NULL,
    capacity INTEGER NOT NULL
);