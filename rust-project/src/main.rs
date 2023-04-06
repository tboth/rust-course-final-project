#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test_page() -> &'static str {
    "This is test page"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, test_page])
}