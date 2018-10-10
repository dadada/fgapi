use schema::doors;
use std::time::SystemTime;
use rocket::request::FromParam;

pub mod handler;
pub mod repository;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "doors"]
pub struct Door {
    pub id: String,
    pub open: bool,
    pub last_change: SystemTime,
}

#[derive(Insertable, Debug)]
#[table_name = "doors"]
pub struct NewDoor<'a> {
    pub id: &'a str,
    pub open: &'a bool,
}

#[derive(Deserialize)]
pub struct DoorCreateForm {
    pub id: String,
    pub open: bool,
}

#[derive(Deserialize)]
pub struct DoorUpdateForm {
    open: bool,
}

#[derive(AsChangeset)]
#[table_name="doors"]
pub struct DoorUpdate<'a> {
    id: Option<&'a str>,
    open: Option<&'a bool>,
    changed: Option<&'a SystemTime>,
}
