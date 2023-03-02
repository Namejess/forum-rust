use crate::schema::messages;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use diesel::sql_types::{Integer, Unsigned};
use dotenv::dotenv;
use std::env;

use diesel::{prelude::*, result::Error};
use serde::{Deserialize, Serialize};

// Définition du modèle de données
mod models;
mod schema;
use models::{Message, NewMessage};
use schema::messages::dsl::*;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Erreur de connexion à {}", database_url))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8001")?
        .run()
        .await
}
