-- This file should undo anything in `up.sql`
CREATE TABLE block_heights (
    id SERIAL PRIMARY KEY,
    height INTEGER NOT NULL
);

DROP TABLE IF EXISTS block_info;
