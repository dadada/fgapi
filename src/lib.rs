#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod connection;
pub mod doors;
mod schema;

use connection::init_pool;

pub fn start() {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_url = format!("{}{}", db_url, "fgapi_test");
    let pool = init_pool(&db_url);

    rocket::ignite()
        .manage(pool)
        .mount(
            "/v1/",
            routes![
                doors::handler::index,
                doors::handler::list_doors,
                doors::handler::get_door,
                doors::handler::post_door
            ],
        ).launch();
}
