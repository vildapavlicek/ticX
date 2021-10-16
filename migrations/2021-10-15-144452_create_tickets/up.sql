-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tickets
(
    id          SERIAL PRIMARY KEY,
    author_id   integer REFERENCES users NOT NULL,
    description VARCHAR     NOT NULL,
    severity    smallint    NOT NULL,
    status      smallint    NOT NULL,
    created     TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);