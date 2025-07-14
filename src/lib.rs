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
    user_id: i32,
    session_token: [u8; 32],
}

impl LoginResponse {
    pub fn new(user_id: i32, session_token: [u8; 32]) -> Self {
        Self {
            user_id,
            session_token,
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
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserSession {
    user_id: i32,
    session_token: [u8; 32],
}

/// Common information which will get displayed on the client side
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserInformation {
    pub username: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogoutReponse {
    
}