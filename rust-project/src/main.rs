use std::fs::File;

use rocket_dyn_templates::{Template, context};
// use rocket_static_files::StaticFiles;
// use rocket_contrib::serve::StaticFiles;
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {field: "value"})
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
        .mount("/", routes![index, test_page])
}
