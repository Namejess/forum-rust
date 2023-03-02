use crate::schema::messages;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Unsigned};
use dotenv::dotenv;
use std::env;

mod schema;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Erreur de connexion à {}", database_url))
}

#[derive(Queryable)]
struct Message {
    id: Unsigned<Integer>,
    content: String,
    author: String,
}

#[derive(Insertable)]
#[table_name = "messages"]
struct NewMessage<'a> {
    content: &'a str,
    author: &'a str,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
