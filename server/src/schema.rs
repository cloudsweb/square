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
        parent_id -> Nullable<Uuid>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        alias -> Text,
        name -> Text,
        description -> Nullable<Text>,
        avatar -> Nullable<Text>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
