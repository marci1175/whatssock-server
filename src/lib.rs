use diesel::{PgConnection, r2d2::ConnectionManager};

pub mod api;
pub mod models;
pub mod schema;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub pg_pool: PgPool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginResponse {
    pub user_id: i32,
    pub session_token: [u8; 32],
    pub chatrooms_joined: Vec<Option<i32>>,
}

impl LoginResponse {
    pub fn new(user_id: i32, session_token: [u8; 32], chatrooms_joined: Vec<Option<i32>>) -> Self {
        Self {
            user_id,
            session_token,
            chatrooms_joined,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

/// This is a UserSession on the Clientside its only named differently cuz of the other struct's naming.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserSession {
    user_id: i32,
    session_token: [u8; 32],
}

/// Common information which will get displayed on the client side
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserInformation {
    pub username: String,
    chatrooms_joined: Vec<Option<i32>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogoutReponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FetchUnknownChatroom {
    pub user_session: UserSession,
    pub chatroom_id: String,
    pub password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FetchKnownChatrooms {
    pub user_session: UserSession,
    pub chatroom_uids: Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FetchChatroomResponse {
    pub chatroom_id: String,
    pub chatroom_name: String,
    /// The reason it is an option is because this is what diesel returns
    pub participants: Vec<Option<i32>>,
    pub is_direct_message: bool,
    pub last_message_id: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateChatroomRequest {
    pub user_session: UserSession,
    pub chatroom_name: String,
    pub chatroom_passw: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FetchKnownChatroomResponse {
    pub chatrooms: Vec<FetchChatroomResponse>,
}
