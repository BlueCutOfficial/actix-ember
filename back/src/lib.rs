use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: String,
    login: String,
    name: String,
}

#[get("/api/users/{login}")]
async fn user(_login: web::Path<String>) -> Result<impl Responder> {
    // hard coded user for testing
    let obj = User {
        id: String::from("bluecut"),
        login: String::from("bluecut@test.com"),
        name: String::from("BlueCut"),
    };
    Ok(web::Json(obj))
}
