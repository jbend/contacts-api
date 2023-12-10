-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts (
    id UUID PRIMARY KEY,
    name varchar NOT NULL,
    email varchar NOT NULL,
    phone varchar NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL,
    updates_at TIMESTAMPTZ NOT NULL,
    UNIQUE (email)
);
