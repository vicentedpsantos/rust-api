#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket_contrib::json::JsonValue;

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> JsonValue {
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

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<Self> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<Self> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
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
        .launch()
        .await;
}
