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

CREATE TABLE transactions (
                              id SERIAL PRIMARY KEY,
                              block_height INT NOT NULL,
                              hash VARCHAR NOT NULL,
                              btc DOUBLE PRECISION NOT NULL,
                              fee BIGINT NOT NULL,
                              time BIGINT NOT NULL,
                              FOREIGN KEY (block_height) REFERENCES block_info (height)
);

CREATE TABLE transaction_inputs (
                                    id SERIAL PRIMARY KEY,
                                    transaction_id INT NOT NULL,
                                    previous_output VARCHAR NOT NULL,
                                    value BIGINT NOT NULL,
                                    FOREIGN KEY (transaction_id) REFERENCES transactions (id)
);

CREATE TABLE transaction_outputs (
                                     id SERIAL PRIMARY KEY,
                                     transaction_id INT NOT NULL,
                                     address VARCHAR NOT NULL,
                                     value BIGINT NOT NULL,
                                     FOREIGN KEY (transaction_id) REFERENCES transactions (id)
);

DROP TABLE IF EXISTS offchain_data;

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



