use rocket_contrib::Json;
use models::{Door};

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/doors/<door>")]
fn get_door(door: String) -> Option<Json<Door>> {
    println!("{:?}", door);
    let door : Door = Door { id : "iz150".to_string(), open: false };
    Some(Json(door))
}

#[post("/doors/<id>", format = "application/json", data = "<door>")]
fn post_door(id : String, door : Json<Door>) ->  Option<Json<Door>> {
    Some(door)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_hello_world() {
        assert_eq!("Hello, world!", index())
    }

    #[test]
    fn get_door() {
        let door = super::get_door("iz150".to_string());
        if let Some(door) = door {
            assert_eq!("iz150", door.id)
        }
    }

    #[test]
    fn post_door() {
        let iz150 = Door { id : "iz150".to_string(), open: false };
        let accepted = super::post_door("iz150".to_string(), Json(iz150));
        if let Some(i) = accepted {
            assert_eq!("iz150", i.id)
        }
    }
}

pub fn start() {
    rocket::ignite().mount("/v1/", routes![index, get_door, post_door]).launch();
}
