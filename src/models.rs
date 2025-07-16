/// These structs contain all the types which are available for insertion in the db.
/// lib.rs contains the types which are necessary for the REST API.
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable, QueryableByName}, Selectable
};

#[derive(Debug, Clone, Selectable, QueryableByName, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccountEntry {
    pub id: i32,
    pub username: String,
    pub passw: String,
    pub email: String,
    pub created_at: chrono::NaiveDate,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserAccount {
    pub username: String,
    pub passw: String,
    pub email: String,
}

#[derive(Debug, Clone, Selectable, QueryableByName, Queryable)]
#[diesel(table_name = crate::schema::user_signin_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSessionEntry {
    pub token_id: i32,
    pub user_id: i32,
    pub session_token: Vec<u8>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::user_signin_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserSession {
    pub user_id: i32,
    pub session_token: Vec<u8>,
}

#[derive(Debug, Clone, Selectable, QueryableByName, Queryable)]
#[diesel(table_name = crate::schema::chatrooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatroomEntry {
    pub id: i32,
    pub chatroom_id: String,
    pub chatroom_name: String,
    pub participants: Vec<Option<i32>>,
    pub is_direct_message: bool,
    pub last_message_id: Option<i32>
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::chatrooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChatroom {
    pub chatroom_id: String,
    pub participants: Vec<i32>,
    pub is_direct_message: bool,
    pub last_message_id: Option<i32>,
}