use schema::doors;
use std::time::SystemTime;

pub mod handler;
pub mod repository;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "doors"]
pub struct Door {
    pub id: String,
    pub open: bool,
    pub last_change: SystemTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "doors"]
pub struct NewDoor<'a> {
    pub id: &'a str,
}
