CREATE TABLE IF NOT EXISTS participants
(
    user_id          INT       NOT NULL,
    conversation_id  INT       NOT NULL,
    messages_read_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (user_id, conversation_id)
);
