use database::article;
use rocket_dyn_templates::{Template, context};
use rocket::{fs::FileServer, form::Form};
use rocket::State;
use sea_orm::*;
use serde::{Serialize};
use crate::database::article::Model as Article;
use crate::database::user::Model as User;

pub mod database;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

#[macro_use] extern crate rocket;

#[derive(Serialize)]
struct Post {
    title: String,
    excerpt: String,
    id: u16
}

#[derive(Serialize)]
struct Posts {
    posts: Vec<Post>
}

#[get("/")]
async fn index(db: &State<DatabaseConnection>) -> Template {
    

    let all_articles = Article::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        r#"SELECT * FROM "article"; "#,
        [],
    )).all(db.inner()).await;

    println!("{:?}", all_articles);

    let context = Posts{
            posts: vec! [
                Post {
                    title: "My Test title".to_string(),
                    excerpt: "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Ducimus perferendis at praesentium ipsam expedita nemo temporibus deleniti? Nam enim ex ut illum voluptas voluptatem, unde cum totam quae optio soluta!...".to_string(),
                    id: 1
                },
                Post {
                    title: "My Test title 2".to_string(),
                    excerpt: "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Ducimus perferendis at praesentium ipsam expedita nemo temporibus deleniti? Nam enim ex ut illum voluptas voluptatem, unde cum totam quae optio soluta!...".to_string(),
                    id: 2
                },
            ],
        };
    Template::render("index", context)
}

#[get("/login")]
fn login(db: &State<DatabaseConnection>) -> Template {
    Template::render("login", context! {field: "value"})
}

#[get("/register")]
fn register(db: &State<DatabaseConnection>) -> Template {
    Template::render("register", context! {field: "value"})
}

#[get("/post/<id>")]
fn post(id: u16, db: &State<DatabaseConnection>) -> Template {
    let context = context!{
        title: format!("My Test title {}", id),
        content: [
            "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Ducimus perferendis at praesentium ipsam expedita nemo temporibus deleniti? Nam enim ex ut illum voluptas voluptatem, unde cum totam quae optio soluta!",
            "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Ducimus perferendis at praesentium ipsam expedita nemo temporibus deleniti? Nam enim ex ut illum voluptas voluptatem, unde cum totam quae optio soluta!",
            "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Ducimus perferendis at praesentium ipsam expedita nemo temporibus deleniti? Nam enim ex ut illum voluptas voluptatem, unde cum totam quae optio soluta!"
        ],
        image: "https://picsum.photos/400/300",
        author: "testUser"
    };
    Template::render("post", context)
}

#[get("/addpost")]
fn addpost(db: &State<DatabaseConnection>) -> Template {
    Template::render("addpost", context! {field: "value"})
}

#[get("/test")]
fn test_page(db: &State<DatabaseConnection>) -> &'static str {
    "This is test page"
}

#[derive(FromForm)]
struct LoginForm<'a> {
    name: &'a str,
    password: &'a str
}

#[post("/login", data = "<user_input>")]
fn api_login(user_input: Form<LoginForm>) -> String {
    format!("Your value: name - {}, password - {}", user_input.name, user_input.password)
}

#[derive(FromForm)]
struct RegisterForm<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
    password_again: &'a str,
}

#[post("/register", data = "<user_input>")]
fn api_register(user_input: Form<RegisterForm>) -> String {
    format!("Your value: name - {}, email - {}, password - {}, password again - {}", 
    user_input.name, user_input.email, user_input.password, user_input.password_again)
}

#[derive(FromForm)]
struct AddPostForm<'a> {
    title: &'a str,
    image: &'a str,
    content: &'a str,
}

#[post("/addpost", data = "<user_input>")]
fn api_addpost(user_input: Form<AddPostForm>) -> String {
    format!("Your value: title - {}, image - {}, content - {}",
    user_input.title, user_input.image, user_input.content)
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
        .mount("/api", routes![api_login, api_register, api_addpost])
}
