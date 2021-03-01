#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::status;
use rocket_contrib::json::JsonValue;

#[get("/rustaceans")]
fn get_rustaceans() -> JsonValue {
    json!([{ "id": 1, "name": "Vicente Santos" }, { "id": 2, "name": "Tamires Quito" }])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> JsonValue {
    json!({ "id": id, "name": "Vicente Santos" })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> JsonValue {
    json!({ "id": 3, "name": "Vicente Santos" })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> JsonValue {
    json!({ "id": id, "name": "Vicente Santos" })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register(catchers![not_found])
        .launch()
        .await;
}
