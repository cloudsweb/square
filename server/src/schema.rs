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
    }
}

diesel::table! {
    secrets (id) {
        id -> Int8,
        current -> Text,
        salt -> Text,
        updated_ip -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        alias -> Text,
        name -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        region -> Nullable<Text>,
        description -> Nullable<Text>,
        avatar -> Nullable<Text>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(posts -> users (author_id));
diesel::joinable!(secrets -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    secrets,
    users,
);
