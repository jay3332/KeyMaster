-- Add migration script here
CREATE TABLE IF NOT EXISTS auth_sessions (
    user_id BIGINT PRIMARY KEY,
    token TEXT NOT NULL
);