-- Your SQL goes here
CREATE TABLE offchain_data (
                               id SERIAL PRIMARY KEY,
                               block_height INTEGER NOT NULL,
                               btc_price DOUBLE PRECISION NOT NULL,
                               market_sentiment DOUBLE PRECISION,
                               volume DOUBLE PRECISION,
                               high DOUBLE PRECISION,
                               low DOUBLE PRECISION,
                               timestamp TIMESTAMP NOT NULL
);
