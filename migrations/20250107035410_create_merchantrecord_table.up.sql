-- Add up migration script here
CREATE TABLE IF NOT EXISTS MerchantRecord (
    id UUID PRIMARY KEY,
    embedding_hash VARCHAR NOT NULL,
    merchant_id INTEGER NOT NULL,
    data_issued TIMESTAMP NOT NULL,
    valid_until TIMESTAMP,
    prev_data_hash VARCHAR NOT NULL,
    data_record JSONB NOT NULL
);
