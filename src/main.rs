#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate diesel;

mod auth;

use auth::BasicAuth;
use rocket::response::status;
use rocket_contrib::json::JsonValue;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth, _conn: DbConn) -> JsonValue {
    json!([{ "id": 1, "name": "Vicente Santos" }, { "id": 2, "name": "Tamires Quito" }])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> JsonValue {
    json!({ "id": id, "name": "Vicente Santos" })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> JsonValue {
    json!({ "id": 3, "name": "Vicente Santos" })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> JsonValue {
    json!({ "id": id, "name": "Vicente Santos" })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": 404,
        "title": "Not Found",
        "description": "The resource couldn't be found"
    })
}

#[catch(401)]
fn unauthorized() -> JsonValue {
    json!({
        "status": 401,
        "title": "Unauthorized",
        "description": "The request requires user authentication"
    })
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
        .register(catchers![not_found, unauthorized])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
