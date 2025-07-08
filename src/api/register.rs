use crate::api::register::users::dsl::users;
use crate::models::{NewUserAccount, NewUserSession, UserAccount, UserSession};
use crate::schema::user_signin_tokens::dsl::user_signin_tokens;
use crate::schema::user_signin_tokens::user_id;
use crate::schema::users::{passw, username};
use crate::{LoginRequest, LoginResponse, RegisterRequest, ServerState, schema::*};
use axum::{Json, extract::State, http::StatusCode};
use diesel::dsl::count_star;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};
use log::error;
use rand::{Rng, rng};

#[axum::debug_handler]
pub async fn fetch_login(
    State(state): State<ServerState>,
    Json(information): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user_account = users
        .filter(username.eq(information.username.clone()))
        .filter(passw.eq(information.password))
        .select(UserAccount::as_select())
        .get_result(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while searching for the user's account: {}",
                err.to_string()
            );

            StatusCode::NOT_FOUND
        })?;

    // Issue a new session token for future logins
    let session_token = generate_session_token();

    // Search up a session token for the user, if it exists delete it
    diesel::delete(user_signin_tokens.filter(user_id.eq(user_account.id)))
        .execute(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while searching for the user's session token: {}",
                err.to_string()
            );

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Store the new session token in the db
    diesel::insert_into(user_signin_tokens)
        .values(&NewUserSession {
            user_id: user_account.id,
            session_token: session_token.clone().to_vec(),
        })
        .get_result::<UserSession>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        user_id: user_account.id,
        session_token: session_token,
    }))
}

pub async fn register_user(
    State(state): State<ServerState>,
    Json(information): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user_count = users
        .filter(username.eq(information.username.clone()))
        .select(count_star())
        .first::<i64>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::REQUEST_TIMEOUT
        })?;

    if user_count != 0 {
        return Err(StatusCode::FOUND);
    }

    // Insert the user's register information into the DB
    let user_account = diesel::insert_into(users)
        .values(&NewUserAccount {
            username: information.username.clone(),
            passw: information.password,
            email: information.email,
        })
        .get_result::<UserAccount>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Issue a new session token for future logins
    let session_token = generate_session_token();

    // Store the session token in the db, there is no way of having another session token for this user as we have just created it.
    diesel::insert_into(user_signin_tokens)
        .values(&NewUserSession {
            user_id: user_account.id,
            session_token: session_token.clone().to_vec(),
        })
        .get_result::<UserSession>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        user_id: user_account.id,
        session_token,
    }))
}

pub fn generate_session_token() -> [i16; 16] {
    let mut rng = rng();

    let mut custom_identifier = [0 as i16; 16];

    rng.fill(&mut custom_identifier);

    custom_identifier
}
