#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;

extern crate rocket;

pub mod schema;
pub mod models;
pub mod api;

extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::{Door, NewDoor};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_door<'a>(conn : &PgConnection, name: &'a str) -> Door {
    use schema::doors;

    let new_door = NewDoor {
        id : name,
    };

    diesel::insert_into(doors::table)
        .values(&new_door)
        .get_result(conn)
        .expect("Error saving new door")
}
