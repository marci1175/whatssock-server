// @generated automatically by Diesel CLI.

diesel::table! {
    chatrooms (id) {
        id -> Int4,
        chatroom_id -> Varchar,
        chatroom_name -> Varchar,
        chatroom_password -> Nullable<Varchar>,
        participants -> Array<Nullable<Int4>>,
        is_direct_message -> Bool,
        last_message_id -> Nullable<Int4>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        parent_chatroom_id -> Int4,
        owner_user_id -> Int4,
        send_date -> Timestamp,
        raw_message -> Bytea,
    }
}

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
        chatrooms_joined -> Array<Nullable<Int4>>,
        created_at -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chatrooms,
    messages,
    posts,
    user_signin_tokens,
    users,
);
