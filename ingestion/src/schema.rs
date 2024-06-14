// @generated automatically by Diesel CLI.

diesel::table! {
    block_height (id) {
        id -> Int4,
        height -> Nullable<Int4>,
    }
}

diesel::table! {
    block_heights (id) {
        id -> Int4,
        height -> Int4,
    }
}

diesel::table! {
    block_info (id) {
        id -> Int4,
        height -> Int4,
        avg_tx_count -> Int4,
        difficulty -> Float8,
        block_time -> Int4,
        timestamp -> Timestamp,
        size -> Int4,
        weight -> Int4,
    }
}

diesel::table! {
    offchain_data (id) {
        id -> Int4,
        block_height -> Int4,
        btc_price -> Float8,
        market_sentiment -> Float8,
        volume -> Float8,
        high -> Float8,
        low -> Float8,
        timestamp -> Timestamp,
    }
}


diesel::table! {
    transaction_inputs (id) {
        id -> Int4,
        transaction_id -> Int4,
        previous_output -> Varchar,
        value -> Int8,
    }
}

diesel::table! {
    transaction_outputs (id) {
        id -> Int4,
        transaction_id -> Int4,
        address -> Varchar,
        value -> Int8,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        block_height -> Int4,
        hash -> Varchar,
        btc -> Float8,
        fee -> Int8,
        time -> Int8,
    }
}

diesel::joinable!(transaction_inputs -> transactions (transaction_id));
diesel::joinable!(transaction_outputs -> transactions (transaction_id));

diesel::allow_tables_to_appear_in_same_query!(
    block_height,
    block_heights,
    block_info,
    offchain_data,
    transaction_inputs,
    transaction_outputs,
    transactions,
);
