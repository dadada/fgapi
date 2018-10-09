use connection::DbConn;
use diesel::result::QueryResult;
use doors::repository::*;
use doors::Door;
use rocket_contrib::Json;

#[get("/", format = "application/json")]
pub fn index() -> Json<&'static str> {
    Json("Hello, world!")
}

#[get("/doors", format = "application/json")]
pub fn list_doors(conn: DbConn) -> QueryResult<Json<Vec<Door>>> {
    all(conn).map(|doors| Json(doors))
}

#[get("/doors/<door>", format = "application/json")]
pub fn get_door(conn: DbConn, door: String) -> QueryResult<Json<Door>> {
    get(conn, door).map(|door| Json(door))
}

#[post("/doors", format = "application/json", data = "<new_door>")]
pub fn post_door(conn: DbConn, new_door: Json<Door>) -> QueryResult<Json<Door>> {
    create(conn, new_door.into_inner()).map(|door| Json(door))
}

#[cfg(test)]
mod tests {
    use super::*;
    use connection::establish_test_connection;
    use diesel::RunQueryDsl;
    use schema::doors::dsl::*;
    use std::panic;
    use connection::init_pool;
    use std::time::SystemTime;
    use std::env;
    use dotenv::dotenv;

    fn setup() -> DbConn {
        dotenv().ok();

        let mut db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let db_url = format!("{}{}", db_url, "fgapi_test");
        let pool = init_pool(db_url);
        let conn = pool.get().expect("Failed to establish database connection");
        diesel::delete(doors).execute(&*conn).expect("Failed to delete doors");
        DbConn(conn)
    }

    #[test]
    fn index_hello_world() {
        assert_eq!("Hello, world!", index().into_inner())
    }

    #[test]
    fn get_door() {
        let conn = setup();
        let door = create_from_name(&conn, "iz150");
        let res = super::get_door(conn, "iz150".to_string());
        if let Ok(door) = door {
            assert_eq!("iz150", &door.id)
        }
    }

    #[test]
    fn post_door() {
        let conn = setup();
        let iz150 = Door {
            id: "iz150".to_string(),
            open: false,
            last_change: SystemTime::now(),
        };
        let accepted = super::post_door(conn, Json(iz150));
        if let Ok(i) = accepted {
            assert_eq!("iz150", i.id)
        }
    }
}
