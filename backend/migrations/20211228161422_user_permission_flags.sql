-- Add migration script here
ALTER TABLE users ADD COLUMN IF NOT EXISTS permissions BIGINT NOT NULL DEFAULT 0