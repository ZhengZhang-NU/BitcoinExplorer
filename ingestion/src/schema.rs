// @generated automatically by Diesel CLI.

diesel::table! {
    block_height (id) {
        id -> Int4,
        height -> Nullable<Int4>,
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

diesel::allow_tables_to_appear_in_same_query!(
    block_height,
    block_info,
);
