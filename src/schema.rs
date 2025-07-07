// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passw -> Varchar,
        email -> Varchar,
        gender -> Bool,
        created_at -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(posts, users,);
