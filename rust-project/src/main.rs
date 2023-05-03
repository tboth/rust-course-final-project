use rocket_dyn_templates::{Template};
use rocket::{fs::FileServer};
use sea_orm::*;
use crate::api_endpoints::*;
use crate::templates::*;

pub mod database;
pub mod templates;
pub mod api_endpoints;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

#[macro_use] extern crate rocket;


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
        .mount("/", routes![index, login, register, post, addpost])
        .mount("/api", routes![api_getallposts,api_getuser,api_getpost,api_login, api_register, api_addpost, api_logout, api_check_logged_in])
}
