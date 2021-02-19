use crate::schema::participants;

#[derive(Insertable)]
#[table_name = "participants"]
pub struct NewParticipant<'a> {
	pub user_id: &'a i32,
	pub conversation_id: &'a i32,
}
