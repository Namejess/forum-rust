use crate::schema::messages;
use diesel::mysql::sql_types::Unsigned;
use diesel::{sql_types::Integer, Insertable, Queryable};
//import schema.rs

#[derive(Queryable)]
pub struct Message {
    pub id: Unsigned<Integer>,
    pub content: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub content: &'a str,
    pub author: &'a str,
}
