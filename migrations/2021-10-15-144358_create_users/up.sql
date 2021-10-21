-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    id        SERIAL PRIMARY KEY,
    username  VARCHAR UNIQUE NOT NULL,
    password  VARCHAR        NOT NULL,
    firstname VARCHAR        NOT NULL,
    lastname  VARCHAR        NOT NULL,
    created   TIMESTAMPTZ    NOT NULL DEFAULT CURRENT_TIMESTAMP
);