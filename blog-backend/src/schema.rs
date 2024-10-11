// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (id) {
        id -> Int4,
        post_title -> Text,
        post_category -> Text,
        post_short_description -> Text,
        post_description -> Text,
        post_photo -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blog_posts,
    users,
);
