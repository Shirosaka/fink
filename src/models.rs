use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::starboard_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StarboardEntry {
    pub id: i32,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub reactions: i32,
}