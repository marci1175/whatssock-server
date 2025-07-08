use diesel::{PgConnection, r2d2::ConnectionManager};
use rand::rngs::ThreadRng;

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
    session_token: [i16; 16],
}

impl LoginResponse {
    pub fn new(user_id: i32, session_token: [i16; 16]) -> Self {
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
