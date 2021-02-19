CREATE TABLE IF NOT EXISTS conversations
(
    id              SERIAL PRIMARY KEY,
    last_message_id INT
);
CREATE INDEX IF NOT EXISTS last_message_id ON conversations (last_message_id DESC);
