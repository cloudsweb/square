// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        topic_id -> Int8,
        id -> Uuid,
        floor -> Int4,
        author_id -> Int8,
        author_name -> Text,
        content -> Int4,
        revision -> Int4,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        parent_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Text,
        alias -> Text,
        description -> Nullable<Text>,
        avatar -> Nullable<Text>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
