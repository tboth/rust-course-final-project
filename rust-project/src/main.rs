use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;
use rocket::State;
use sea_orm::*;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

#[macro_use] extern crate rocket;

#[get("/")]
fn index(db: &State<DatabaseConnection>) -> Template {
    Template::render("index", context! {field: "value"})
}

#[get("/login")]
fn login(db: &State<DatabaseConnection>) -> Template {
    Template::render("login", context! {field: "value"})
}

#[get("/register")]
fn register(db: &State<DatabaseConnection>) -> Template {
    Template::render("register", context! {field: "value"})
}

#[get("/post")]
fn post(db: &State<DatabaseConnection>) -> Template {
    Template::render("post", context! {field: "value"})
}

#[get("/addpost")]
fn addpost(db: &State<DatabaseConnection>) -> Template {
    Template::render("addpost", context! {field: "value"})
}

#[get("/test")]
fn test_page(db: &State<DatabaseConnection>) -> &'static str {
    "This is test page"
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, test_page, login, register, post, addpost])
}
