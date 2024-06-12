CREATE TABLE transactions (
                              id SERIAL PRIMARY KEY,
                              block_height INT NOT NULL,
                              hash VARCHAR NOT NULL,
                              btc FLOAT8 NOT NULL,
                              fee INT8 NOT NULL,
                              time INT8 NOT NULL,
                              FOREIGN KEY (block_height) REFERENCES block_info (height)
);

CREATE TABLE transaction_inputs (
                                    id SERIAL PRIMARY KEY,
                                    transaction_id INT NOT NULL,
                                    previous_output VARCHAR NOT NULL,
                                    value INT8 NOT NULL,
                                    FOREIGN KEY (transaction_id) REFERENCES transactions (id)
);

CREATE TABLE transaction_outputs (
                                     id SERIAL PRIMARY KEY,
                                     transaction_id INT NOT NULL,
                                     address VARCHAR NOT NULL,
                                     value INT8 NOT NULL,
                                     FOREIGN KEY (transaction_id) REFERENCES transactions (id)
);
