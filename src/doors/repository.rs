use diesel::result::QueryResult;
use diesel::prelude::*;

use connection::DbConn;
use super::{Door, NewDoor};
use schema::doors::dsl::*;

pub fn all(conn: DbConn) -> QueryResult<Vec<Door>> {
    doors.load::<Door>(&*conn)
}

pub fn get(conn: DbConn, name: String) -> QueryResult<Door> {
    doors.filter(id.eq(name))
        .first::<Door>(&*conn)
}

pub fn create_from_name(conn: &DbConn, door: &str) -> QueryResult<Door> {
    let new_door = NewDoor {
        id: door,
    };
    diesel::insert_into(doors)
        .values(&new_door)
        .get_result(&**conn)
}

pub fn create(conn: DbConn, door: Door) -> QueryResult<Door> {
    create_from_name(&conn, &door.id)
}
