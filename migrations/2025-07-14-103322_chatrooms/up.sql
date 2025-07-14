CREATE TABLE chatrooms (
    id SERIAL PRIMARY KEY,
    chatroom_id VARCHAR NOT NULL,
    participants INT[]
);