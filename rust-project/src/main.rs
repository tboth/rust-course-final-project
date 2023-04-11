use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {field: "value"})
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context! {field: "value"})
}

#[get("/register")]
fn register() -> Template {
    Template::render("register", context! {field: "value"})
}

#[get("/post")]
fn post() -> Template {
    Template::render("post", context! {field: "value"})
}

#[get("/addpost")]
fn addpost() -> Template {
    Template::render("addpost", context! {field: "value"})
}

#[get("/test")]
fn test_page() -> &'static str {
    "This is test page"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, test_page, login, register, post, addpost])
}
