use rocket::{get,post, http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};

// cargo watch -q -c -w src/ -x run
#[macro_use]
extern crate rocket;


#[derive(Serialize,Deserialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
#[derive(Serialize,Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
#[derive(Serialize,Deserialize)]
pub struct UserResponse {
    pub status: String,
    pub username: String,
    pub password: String,
}

#[get("/test")]
pub async fn test() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[get("/login")]
pub async fn login() -> Result<Json<UserResponse>, Status> {
    const USERNAME: &str = "Ferko";
    const PASSWORD: &str = "Ferko123";

    let response_json = UserResponse {
        status: "success".to_string(),
        username: USERNAME.to_string(),
        password: PASSWORD.to_string(),
    };
    Ok(Json(response_json))
}

#[post("/register", format = "json", data = "<body>")]
pub async fn register(body: Json<User>) -> Result<Json<UserResponse>, Status> {
    let USERNAME: &str = &body.username.clone();
    let PASSWORD: &str = &body.password.clone();

    let response_json = UserResponse {
        status: "success".to_string(),
        username: USERNAME.to_string(),
        password: PASSWORD.to_string(),
    };
    Ok(Json(response_json))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![test,register,login])
}
