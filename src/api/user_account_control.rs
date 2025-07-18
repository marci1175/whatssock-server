use crate::api::user_account_control::users::dsl::users;
use crate::models::{
    ChatroomEntry, NewChatroom, NewUserAccount, NewUserSession, UserAccountEntry, UserSessionEntry,
};
use crate::schema::chatrooms::dsl::chatrooms;
use crate::schema::chatrooms::{chatroom_id, chatroom_password};
use crate::schema::user_signin_tokens::dsl::user_signin_tokens;
use crate::schema::user_signin_tokens::{session_token, user_id};
use crate::schema::users::{chatrooms_joined, id, passw, username};
use crate::{
    CreateChatroomRequest, FetchChatroomResponse, FetchKnownChatroomResponse, FetchKnownChatrooms,
    FetchUnknownChatroom, LoginRequest, LoginResponse, LogoutReponse, RegisterRequest, ServerState,
    UserInformation, UserSession,
    schema::{self, *},
};
use axum::{Json, extract::State, http::StatusCode};
use diesel::dsl::count_star;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper, Table, delete};
use log::{error, info};
use rand::distr::Uniform;
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

    let user_session_count = user_signin_tokens
        .filter(user_id.eq(user_account.id))
        .select(count_star())
        .first::<i64>(&mut pg_connection)
        .map_err(|err| {
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
        diesel::update(user_signin_tokens)
            .filter(user_id.eq(user_account.id))
            .set(&NewUserSession {
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
    } else {
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
        chatrooms_joined: user_account.chatrooms_joined,
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
            chatrooms_joined: vec![],
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
        chatrooms_joined: user_account.chatrooms_joined,
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
    let count = user_signin_tokens
        .filter(user_id.eq(session_cookie.user_id))
        .filter(session_token.eq(session_cookie.session_token))
        .select(count_star())
        .first::<i64>(&mut pg_connection)
        .map_err(|err| {
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

    let user_account = users
        .filter(id.eq(session_cookie.user_id))
        .select(UserAccountEntry::as_select())
        .first::<UserAccountEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching user session information from db: {}",
                err.to_string()
            );
            StatusCode::REQUEST_TIMEOUT
        })?;

    Ok(Json(UserInformation {
        username: user_account.username,
        chatrooms_joined: user_account.chatrooms_joined,
    }))
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

    match delete(user_signin_tokens.filter(session_token.eq(session_cookie.session_token)))
        .execute(&mut pg_connection)
    {
        Ok(r_affected) => {
            dbg!(r_affected);
        }
        Err(err) => {
            error!("{err}");

            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Json(LogoutReponse {}))
}

pub async fn fetch_unknown_chatroom(
    State(state): State<ServerState>,
    Json(chatroom_request): Json<FetchUnknownChatroom>,
) -> Result<Json<FetchChatroomResponse>, StatusCode> {
    // Get a db connection from the pool
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let chatrooms_filter = chatrooms.filter(chatroom_id.eq(chatroom_request.chatroom_id));

    let query_result: ChatroomEntry = if let Some(password) = chatroom_request.password {
        let password_filter = chatrooms_filter.filter(chatroom_password.eq(password));

        password_filter
            .select(ChatroomEntry::as_select())
            .first(&mut pg_connection)
            .map_err(|err| {
                error!(
                    "An error occured while fetching chatrooms from db: {}",
                    err.to_string()
                );

                StatusCode::INTERNAL_SERVER_ERROR
            })?
    } else {
        chatrooms_filter
            .select(ChatroomEntry::as_select())
            .first(&mut pg_connection)
            .map_err(|err| {
                error!(
                    "An error occured while fetching chatrooms from db: {}",
                    err.to_string()
                );

                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };

    Ok(Json(FetchChatroomResponse {
        chatroom_id: query_result.chatroom_id,
        chatroom_name: query_result.chatroom_name,
        participants: query_result.participants,
        is_direct_message: query_result.is_direct_message,
        last_message_id: query_result.last_message_id,
    }))
}

pub async fn fetch_known_chatrooms(
    State(state): State<ServerState>,
    Json(bulk_chatrooms_request): Json<FetchKnownChatrooms>,
) -> Result<Json<FetchKnownChatroomResponse>, StatusCode> {
    // Get a db connection from the pool
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Verify user session validness
    let matching_user_tokens = user_signin_tokens
        .filter(schema::user_signin_tokens::user_id.eq(bulk_chatrooms_request.user_session.user_id))
        .filter(
            schema::user_signin_tokens::session_token
                .eq(bulk_chatrooms_request.user_session.session_token),
        )
        .select(count_star())
        .get_result::<i64>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while verifying login information from db: {}",
                err.to_string()
            );

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if matching_user_tokens != 1 {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut verified_chatrooms_reponses: Vec<FetchChatroomResponse> = Vec::new();

    // Verify that the user is indeed present in the chatroom
    for chatroom_request in bulk_chatrooms_request.chatroom_uids {
        let chatroom_entry = chatrooms
            .filter(schema::chatrooms::id.eq(chatroom_request))
            .get_result::<ChatroomEntry>(&mut pg_connection)
            .map_err(|err| {
                error!(
                    "An error occured while fetching chatrooms from db: {}",
                    err.to_string()
                );

                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let is_user_present = chatroom_entry
            .participants
            .contains(&Some(bulk_chatrooms_request.user_session.user_id));

        // If the user is not present in the participants list, return an error
        if !is_user_present {
            return Err(StatusCode::FORBIDDEN);
        }

        verified_chatrooms_reponses.push(FetchChatroomResponse {
            chatroom_id: chatroom_entry.chatroom_id,
            chatroom_name: chatroom_entry.chatroom_name,
            participants: chatroom_entry.participants,
            is_direct_message: chatroom_entry.is_direct_message,
            last_message_id: chatroom_entry.last_message_id,
        });
    }

    Ok(Json(FetchKnownChatroomResponse {
        chatrooms: verified_chatrooms_reponses,
    }))
}

pub async fn create_chatroom(
    State(state): State<ServerState>,
    Json(chatroom_request): Json<CreateChatroomRequest>,
) -> Result<Json<FetchChatroomResponse>, StatusCode> {
    // Get a db connection from the pool
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let generated_chatroom_id: String = rand::rng()
        .sample_iter(&Uniform::new(char::from(32), char::from(126)).unwrap())
        .take(10)
        .map(char::from)
        .collect();

    let chatroom_entry: ChatroomEntry = diesel::insert_into(chatrooms)
        .values(&NewChatroom {
            chatroom_id: generated_chatroom_id,
            chatroom_name: chatroom_request.chatroom_name,
            chatroom_password: chatroom_request.chatroom_passw,
            // Insert the user_id into the participants list
            participants: vec![chatroom_request.user_session.user_id],
            is_direct_message: false,
            last_message_id: None,
        })
        .get_result(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while creating a new chatroom: {}",
                err.to_string()
            );

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let mut user_account = users
        .filter(id.eq(chatroom_request.user_session.user_id))
        .get_result::<UserAccountEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching user account with id {}: {}",
                chatroom_request.user_session.user_id,
                err.to_string()
            );

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    user_account.chatrooms_joined.push(Some(chatroom_entry.id));

    diesel::update(users.filter(id.eq(chatroom_request.user_session.user_id)))
        .set(chatrooms_joined.eq(user_account.chatrooms_joined))
        .get_result::<UserAccountEntry>(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while fetching user account with id {}: {}",
                chatroom_request.user_session.user_id,
                err.to_string()
            );

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(FetchChatroomResponse {
        chatroom_id: chatroom_entry.chatroom_id,
        chatroom_name: chatroom_entry.chatroom_name,
        participants: chatroom_entry.participants,
        is_direct_message: chatroom_entry.is_direct_message,
        last_message_id: chatroom_entry.last_message_id,
    }))
}

pub fn generate_session_token() -> [u8; 32] {
    let mut rng = rng();

    let mut custom_identifier = [0 as u8; 32];

    rng.fill(&mut custom_identifier);

    custom_identifier
}
