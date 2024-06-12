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
