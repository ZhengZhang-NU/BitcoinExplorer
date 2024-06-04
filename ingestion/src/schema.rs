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

diesel::allow_tables_to_appear_in_same_query!(
    block_height,
    block_heights,
);
