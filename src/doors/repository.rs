use std::time::SystemTime;
use diesel::result::QueryResult;
use diesel::prelude::*;
use connection::DbConn;
use super::{Door, NewDoor, DoorCreateForm, DoorUpdateForm, DoorUpdate};
use schema::doors::dsl::*;

pub fn all(conn: DbConn) -> QueryResult<Vec<Door>> {
    doors.load::<Door>(&*conn)
}

pub fn get(conn: DbConn, name: String) -> QueryResult<Door> {
    doors.filter(id.eq(name))
        .first::<Door>(&*conn)
}

pub fn create_from_name(conn: &DbConn, door: &str) -> QueryResult<Door> {
    let open_or_not = false;
    let new_door = NewDoor {
        id: door,
        open: &open_or_not,
    };
    diesel::insert_into(doors)
        .values(&new_door)
        .get_result(&**conn)
}

pub fn create(conn: DbConn, door: DoorCreateForm) -> QueryResult<Door> {
    let new_door = NewDoor {
        id: &door.id,
        open: &door.open,
    };
    diesel::insert_into(doors)
        .values(&new_door)
        .get_result(&*conn)
}

pub fn update(conn: DbConn, door_name: String, form: DoorUpdateForm) -> QueryResult<Door> {
    let door = doors.filter(id.eq(door_name))
        .first::<Door>(&*conn).expect("No such door");
    let now = SystemTime::now();
    let update = DoorUpdate {
        id: None,
        open: Some(&form.open),
        changed: Some(&now),
    };
    diesel::update(&door)
        .set(update)
        .get_result(&*conn)
}
