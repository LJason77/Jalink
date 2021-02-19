table! {
	conversations (id) {
		id -> Int4,
		last_message_id -> Nullable<Int4>,
	}
}

table! {
	messages (id) {
		id -> Int4,
		content -> Varchar,
		user_id -> Int4,
		conversation_id -> Int4,
		created_at -> Timestamp,
	}
}

table! {
	participants (user_id, conversation_id) {
		user_id -> Int4,
		conversation_id -> Int4,
		messages_read_at -> Timestamp,
	}
}

table! {
	users (id) {
		id -> Int4,
		username -> Varchar,
		avatar_url -> Nullable<Varchar>,
		github_id -> Int4,
	}
}

joinable!(messages -> users (user_id));

allow_tables_to_appear_in_same_query!(conversations, messages, participants, users,);
