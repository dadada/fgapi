use connection::DbConn;
use diesel::result::QueryResult;
use doors::repository::*;
use doors::{Door, DoorCreateForm, DoorUpdateForm};
use rocket_contrib::Json;

#[get("/")]
pub fn index() -> Json<&'static str> {
    Json("Hello, world!")
}

#[get("/doors")]
pub fn list_doors(conn: DbConn) -> QueryResult<Json<Vec<Door>>> {
    all(conn).map(|doors| Json(doors))
}

#[get("/doors/<door>")]
pub fn get_door(conn: DbConn, door: String) -> QueryResult<Json<Door>> {
    get(conn, door).map(|door| Json(door))
}

#[post("/doors", format = "application/json", data = "<new_door>")]
pub fn post_door(conn: DbConn, new_door: Json<DoorCreateForm>) -> QueryResult<Json<Door>> {
    create(conn, new_door.into_inner()).map(|door| Json(door))
}

#[patch("/doors/<door>", format = "application/json", data = "<form>")]
pub fn patch_door(conn: DbConn, door: String, form: Json<DoorUpdateForm>) -> QueryResult<Json<Door>> {
    update(conn, door, form.into_inner())
        .map(|door| Json(door))
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::RunQueryDsl;
    use schema::doors::dsl::*;
    use std::panic;
    use connection::init_pool;
    use std::time::SystemTime;
    use std::env;
    use dotenv::dotenv;

    fn setup() -> DbConn {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let pool = init_pool(&db_url);
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
        create_from_name(&conn, "iz150").expect("Failed to create door");
        let res = super::get_door(conn, "iz150".to_string());
        if let Ok(door) = res {
            assert_eq!("iz150", &door.id)
        }
    }

    #[test]
    fn post_door() {
        let conn = setup();
        let form = DoorCreateForm {
            id: "iz150".to_string(),
            open: false,
        };
        let accepted = super::post_door(conn, Json(form));
        if let Ok(i) = accepted {
            assert_eq!("iz150", i.id)
        }
    }

    #[test]
    fn patch_door() {
        let conn = setup();
        post_door();
        let form = DoorUpdateForm {
            open: true,
        };
        assert!(super::update(conn, "iz150".to_string(), form).is_ok());
    }
}
