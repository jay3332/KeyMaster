-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    discriminator SMALLINT NOT NULL,
    email TEXT,
    password TEXT NOT NULL  -- Passwords will be hashed
);