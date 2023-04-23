use database::article;
use rocket_dyn_templates::{Template, context};
use rocket::{fs::FileServer, form::Form,http::Status};
use rocket::State;
use sea_orm::*;
use serde::{Serialize, Deserialize};
use crate::database::article::Model as Article;
use crate::database::user::Model as User;
use serde_json::json;


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

    //println!("{:?}", all_articles);
    match all_articles {
        Ok(articles) => {
            for article in articles {
                println!("ID: {}", article.id);
                println!("Username: {}", article.title);
            }
        }
        Err(err) => {
            println!("Error fetching articles: {}", err);
        }
    }
    

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
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////




#[derive(FromForm, Serialize, Deserialize)]
struct LoginForm {
    name: String,
    password: String
}

#[derive(FromForm, Serialize, Deserialize)]
struct PostForm{
    text: String,
    picture: String,
    user_id: i32,
    title: String,
}

#[derive(FromForm, Serialize, Deserialize)]
struct UserForm {
    name: String,
    full_name: String,
    email: String,
    password: String,
    password_again: String,
}

#[derive(FromForm, Serialize, Deserialize)]
struct UserResponse {
    name: String,
    full_name: String,
    email: String,
    password: String,
}

//logging in
#[post("/login", data = "<user_input>")]
async fn api_login(user_input: Form<LoginForm>, db: &State<DatabaseConnection>) -> String {

    let response = User::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM \"user\" WHERE username = '{}'", user_input.name),
        [],
    )).all(db.inner()).await;

    match response {
        Ok(response) => {
            for column in response {
                if column.password == user_input.password {
                    return format!("Correct password for user {}", column.username);
                }
                else {
                    return format!("Incorrect password for user {}", column.username);
                }
            }
            return format!("User {} not found", user_input.name);
        }
        Err(err) => {
            return format!("Fetching error: {}", err);
        }
    }
}


//getting post by id
#[get("/getPost/<id>")]
async fn api_getpost(id: u16, db: &State<DatabaseConnection>) -> Result<String, Status> {
    let response = Article::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM \"article\" WHERE id = '{}'", id),
        [],
    )).all(db.inner()).await;

    println!("{:?}", response);
    match response {
        Ok(response) => {
            for column in response {
                println!("Article '{}' fetched succesfully", column.title);
                let json_string = serde_json::to_string(&PostForm{
                    text: column.text,
                    picture: column.picture.unwrap_or_else(|| "".to_string()),
                    user_id: column.user_id,
                    title: column.title,
                }).unwrap();
                return Ok(json_string);
            }
            println!("Article with id {} not found", id);
            return Err(Status::NotFound);
        }
        Err(err) => {
            println!("Fetching error: {}", err);
            return Err(Status::BadRequest);
        }
    }
}


#[get("/getUser/<name>")]
async fn api_getuser(name: String, db: &State<DatabaseConnection>) -> Result<String, Status> {
    let response = User::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM \"user\" WHERE username = '{}'", name),
        [],
    )).all(db.inner()).await;

    println!("{:?}", response);
    match response {
        Ok(response) => {
            for column in response {
                println!("User '{}' fetched succesfully", column.username);
                let json_string = serde_json::to_string(&UserResponse{
                    name: column.username,
                    full_name: column.full_name,
                    email: column.email,
                    password: column.password,
                }).unwrap();
                println!("{}", json_string);
                return Ok(json_string);
            }
            println!("User with name {} not found", name);
            return Err(Status::NotFound);
        }
        Err(err) => {
            println!("Fetching error: {}", err);
            return Err(Status::BadRequest);
        }
    }
}

//creating an account
#[post("/register", data = "<user_input>")]
async fn api_register(user_input: Form<UserForm>, db: &State<DatabaseConnection>) -> Status {
    format!("Your value: name - {}, email - {}, password - {}, password again - {}", user_input.name, user_input.email, user_input.password, user_input.password_again);

    if user_input.password == user_input.password_again {
        let response = User::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            &format!("INSERT INTO \"user\" (username, full_name, email, password) VALUES ('{}', '{}', '{}', '{}')", user_input.name,user_input.full_name,user_input.email,user_input.password),
            [],
        )).all(db.inner()).await;
        println!("{:?}", response);
        match response {
            Ok(response) => {
                println!("User {} created.", user_input.name);
                return Status::Created;
            }
            Err(err) => {
                println!("Error creating user: {}", err);
                return Status::BadRequest;
            }
        }
    }
    else {
        println!("Passwords do not match");
        return Status::ExpectationFailed;
    }
}


#[post("/addpost", data = "<user_input>")]
async fn api_addpost(user_input: Form<PostForm>, db: &State<DatabaseConnection>) -> Status {
    let response = Article::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("INSERT INTO \"article\" (text, picture, user_id, title) VALUES ('{}', '{}', '{}', '{}')", user_input.text,user_input.picture,user_input.user_id,user_input.title),
        [],
    )).all(db.inner()).await;

    println!("{:?}", response);
    match response {
        Ok(response) => {
            println!("Article {} created", user_input.title);
            return Status::Created;
        }
        Err(err) => {
            println!("Error creating article: {}", err);
            return Status::BadRequest;
        }
    }
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
        .mount("/api", routes![api_getuser,api_getpost,api_login, api_register, api_addpost])
}
