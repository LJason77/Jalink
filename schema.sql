CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(40) NOT NULL,
    avatar_url VARCHAR(100),
    github_id  INT         NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS conversations
(
    id              SERIAL PRIMARY KEY,
    last_message_id INT
);
CREATE INDEX IF NOT EXISTS last_message_id ON conversations (last_message_id DESC);

CREATE TABLE IF NOT EXISTS participants
(
    user_id          INT       NOT NULL,
    conversation_id  INT       NOT NULL,
    messages_read_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (user_id, conversation_id)
);

CREATE TABLE IF NOT EXISTS messages
(
    id              SERIAL PRIMARY KEY,
    content         VARCHAR(500) NOT NULL,
    user_id         INT          NOT NULL REFERENCES users ON DELETE CASCADE,
    conversation_id INT          NOT NULL REFERENCES conversations ON DELETE CASCADE,
    created_at      TIMESTAMP    NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS created_at ON messages (created_at DESC);

ALTER TABLE conversations
    DROP CONSTRAINT IF EXISTS fk_last_message_id_ref_messages;
ALTER TABLE conversations
    ADD CONSTRAINT fk_last_message_id_ref_messages
        FOREIGN KEY (last_message_id) REFERENCES messages ON DELETE SET NULL;
