-- Add up migration script here
CREATE TABLE IF NOT EXISTS MerchantData (
    merchant_id INTEGER PRIMARY KEY,
    schema JSONB NOT NULL,
    readable_fields TEXT[] NOT NULL
);
