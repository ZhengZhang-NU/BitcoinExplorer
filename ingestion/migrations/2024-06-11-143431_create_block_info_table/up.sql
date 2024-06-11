DROP TABLE IF EXISTS block_heights;

CREATE TABLE block_info (
    id SERIAL PRIMARY KEY,
    height INTEGER NOT NULL,
    avg_tx_count INTEGER NOT NULL,
    difficulty DOUBLE PRECISION NOT NULL,
    block_time INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    size INTEGER NOT NULL,
    weight INTEGER NOT NULL
);
