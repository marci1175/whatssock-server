use crate::api::user_account_control::users::dsl::users;
use crate::models::{NewUserAccount, NewUserSession, UserAccountEntry, UserSessionEntry};
use crate::schema::user_signin_tokens::dsl::user_signin_tokens;
use crate::schema::user_signin_tokens::{session_token, user_id};
use crate::schema::users::{id, passw, username};
use crate::{schema::*, LoginRequest, LoginResponse, LogoutReponse, RegisterRequest, ServerState, UserInformation, UserSession};
use axum::{Json, extract::State, http::StatusCode};
use diesel::dsl::count_star;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{delete, ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper, Table};
use log::{error, info};
use rand::{Rng, rng};

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
        .select(UserAccountEntry::as_select())
        .get_result(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while searching for the user's account: {}",
                err.to_string()
            );

            StatusCode::NOT_FOUND
        })?;

    // Issue a new session token for future logins
    let session_cookie_token = generate_session_token();

    let user_session_count = user_signin_tokens.filter(user_id.eq(user_account.id)).select(count_star()).first::<i64>(&mut pg_connection).map_err(|err| {
        error!(
                "An error occured while searching for the user's session token: {}",
                err.to_string()
            );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Check if there are any existing user sessions
    // If there arent this means some sort of issue has occured, thus the session has been invalidated or deleted.
    if user_session_count != 0 {
        // Search up a session token for the user, if it exists update it
        diesel::update(user_signin_tokens).filter(user_id.eq(user_account.id)).set(&NewUserSession {
            user_id: user_account.id,
            session_token: session_cookie_token.clone().to_vec(),
        })
        .get_result::<UserSessionEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while searching for the user's session token: {}",
                err.to_string()
            );

            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }
    else {
        diesel::insert_into(user_signin_tokens)
        .values(&NewUserSession {
            user_id: user_account.id,
            session_token: session_cookie_token.clone().to_vec(),
        })
        .get_result::<UserSessionEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    Ok(Json(LoginResponse {
        user_id: user_account.id,
        session_token: session_cookie_token,
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
        .get_result::<UserAccountEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Issue a new session token for future logins
    let session_cookie_token = generate_session_token();

    // Store the session token in the db, there is no way of having another session token for this user as we have just created it.
    diesel::insert_into(user_signin_tokens)
        .values(&NewUserSession {
            user_id: user_account.id,
            session_token: session_cookie_token.clone().to_vec(),
        })
        .get_result::<UserSessionEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        user_id: user_account.id,
        session_token: session_cookie_token,
    }))
}

pub async fn fetch_session_token(
    State(state): State<ServerState>,
    Json(session_cookie): Json<UserSession>,
) -> Result<Json<UserInformation>, StatusCode> {
    // Get a db connection from the pool
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get how many fields are equal (This must be one, or zero.)
    let count = user_signin_tokens.filter(user_id.eq(session_cookie.user_id)).filter(session_token.eq(session_cookie.session_token)).select(count_star()).first::<i64>(&mut pg_connection).map_err(|err| {
            error!(
                "An error occured while fetching user session information from db: {}",
                err.to_string()
            );
            StatusCode::REQUEST_TIMEOUT
        })?;
    
    // If the user token is not found return an error indication that it is false.
    if count != 1 {
        return Err(StatusCode::NOT_ACCEPTABLE);
    }

    let user_account = users.filter(id.eq(session_cookie.user_id)).select(UserAccountEntry::as_select()).first::<UserAccountEntry>(&mut pg_connection).map_err(|err| {
            error!(
                "An error occured while fetching user session information from db: {}",
                err.to_string()
            );
            StatusCode::REQUEST_TIMEOUT
        })?;

    Ok(Json(UserInformation { username: user_account.username }))
}

pub async fn handle_logout_request(
    State(state): State<ServerState>,
    Json(session_cookie): Json<UserSession>,
) -> Result<Json<LogoutReponse>, StatusCode> {
    dbg!(&session_cookie);

    // Get a db connection from the pool
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    match delete(user_signin_tokens.filter(session_token.eq(session_cookie.session_token))).execute(&mut pg_connection)  {
        Ok(r_affected) => {
            dbg!(r_affected);
        },
        Err(err) => {
            error!("{err}");

            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    }

    Ok(Json(LogoutReponse {}))
}

pub fn generate_session_token() -> [u8; 32] {
    let mut rng = rng();

    let mut custom_identifier = [0 as u8; 32];

    rng.fill(&mut custom_identifier);

    custom_identifier
}
