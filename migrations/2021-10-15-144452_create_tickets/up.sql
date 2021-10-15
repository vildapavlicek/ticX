-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tickets
(
    id          SERIAL PRIMARY KEY,
    author_id   integer REFERENCES users,
    description VARCHAR     NOT NULL,
    severity    smallint,
    status      smallint,
    created     TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);