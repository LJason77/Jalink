CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(40) NOT NULL,
    avatar_url VARCHAR(100),
    github_id  INT         NOT NULL UNIQUE
);