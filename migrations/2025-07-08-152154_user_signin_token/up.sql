-- Your SQL goes here
CREATE TABLE user_signin_tokens(
    token_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    session_token SMALLINT[32] NOT NULL
);