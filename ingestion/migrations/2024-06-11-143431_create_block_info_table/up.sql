CREATE TABLE block_info (
                            id SERIAL PRIMARY KEY,
                            height INT NOT NULL,
                            avg_tx_count INT NOT NULL,
                            difficulty FLOAT8 NOT NULL,
                            block_time INT NOT NULL,
                            timestamp TIMESTAMP NOT NULL,
                            size INT NOT NULL,
                            weight INT NOT NULL,
                            CONSTRAINT unique_height UNIQUE (height)
);
