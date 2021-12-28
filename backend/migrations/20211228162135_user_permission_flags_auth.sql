-- Add migration script here
ALTER TABLE auth_sessions ADD COLUMN permissions BIGINT NOT NULL DEFAULT 0;