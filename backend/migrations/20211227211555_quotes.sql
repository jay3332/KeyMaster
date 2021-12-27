-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id SERIAL PRIMARY KEY,
    author_id BIGINT,  -- the ID of the user who submitted the quote
    content TEXT NOT NULL,
    author TEXT  -- author is NOT the same author as in author_id; instead it is the quote author rather than the person who submitted the quote.
);