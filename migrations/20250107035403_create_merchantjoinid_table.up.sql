-- Add up migration script here
CREATE TABLE IF NOT EXISTS MerchantJoinId (
    id UUID PRIMARY KEY,
    merchant_id INTEGER NOT NULL,
    embedding_hash VARCHAR NOT NULL,
    write_fields TEXT[] NOT NULL,
    read_merchant_fields JSONB NOT NULL,
    last_data_hash VARCHAR NOT NULL,
    last_updated TIMESTAMP
);
