#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::*;
use repositories::*;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(|c| {
        RustaceanRepository::load_all(c)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        RustaceanRepository::find_one(c, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    conn: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/rustaceans/<_id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    _id: i32,
    _auth: BasicAuth,
    conn: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        RustaceanRepository::save(c, rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Status {
    conn.run(move |c| match RustaceanRepository::delete(c, id) {
        true => Status::NoContent,
        false => Status::NotFound,
    })
    .await
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

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "status": 422,
        "title": "Unprocessable Entity",
        "description": "The request was well-formed but one or more attributes are missing."
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
        .register(catchers![not_found, unauthorized, unprocessable_entity])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
