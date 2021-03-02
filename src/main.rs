#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::*;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use schema::*;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> JsonValue {
    conn.run(|c| {
        let all = rustaceans::table
            .limit(100)
            .load::<Rustacean>(c)
            .expect("Error loading rustaceans from DB");
        json!(all)
    })
    .await
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> JsonValue {
    json!({ "id": id, "name": "Vicente Santos" })
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    conn: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> JsonValue {
    conn.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error adding rustacean to db");
        json!(result)
    })
    .await
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
