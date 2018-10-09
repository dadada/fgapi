#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

pub mod connection;
pub mod doors;
mod schema;

use connection::init_pool;

pub fn start() {
    rocket::ignite()
        .manage(init_pool())
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
