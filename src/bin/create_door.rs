extern crate fgapi;

use fgapi::{establish_connection, create_door};

fn main() {
    let conn = establish_connection();
    let door = create_door(&conn, "iz150");
    println!("{:?}", door)
}
