use schema::{doors};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name="doors"]
pub struct Door {
    pub id : String,
    pub open : bool,
}

#[derive(Insertable, Debug)]
#[table_name="doors"]
pub struct NewDoor<'a> {
    pub id : &'a str,
}
