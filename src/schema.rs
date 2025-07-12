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
    user_signin_tokens (token_id) {
        token_id -> Int4,
        user_id -> Int4,
        session_token -> Bytea,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passw -> Varchar,
        email -> Varchar,
        created_at -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    user_signin_tokens,
    users,
);
