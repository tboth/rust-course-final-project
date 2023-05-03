use rocket_dyn_templates::{Template, context};
use rocket::{fs::FileServer, form::Form, http::Status};
use rocket::State;
use sea_orm::*;
use serde::{Serialize, Deserialize};
use crate::database::article::Model as Article;
use crate::database::user::Model as User;
use rocket::response::Redirect;

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
    id: String
}

#[derive(Serialize)]
struct Posts {
    user_logged_in: i32,
    posts: Vec<Post>
}

#[get("/?<user_id>")]
async fn index(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {  
    let result = api_getallposts(db).await.unwrap();
    let posts: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    let mut context = Posts{
        user_logged_in: 0,
        posts: Vec::new()
    };

    if user_id.is_some() && user_id.unwrap().to_string() != 0.to_string(){
        let is_logged_in = api_check_logged_in(user_id.unwrap(), db).await.unwrap();
        if is_logged_in.to_string() == 1.to_string(){
            context.user_logged_in = user_id.unwrap();
        }
    }

    for post in &posts {
        let title = post["title"].as_str().unwrap().to_string();
        let text = post["text"].as_str().unwrap().to_string();
        context.posts.push(Post{
            title: title,
            excerpt: if text.len() > 200 {(&text[0..200]).to_string()} else {text},
            id: post["id"].to_string()
        })
    };

    Template::render("index", context)
}

#[get("/login?<user_id>")]
async fn login(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
    let mut context = context!{
        user_logged_in: 0
    };

    if user_id.is_some() && user_id.unwrap().to_string() != 0.to_string(){
        let is_logged_in = api_check_logged_in(user_id.unwrap(), db).await.unwrap();
        if is_logged_in.to_string() == 1.to_string(){
            context.user_logged_in = user_id.unwrap();
        }
    }

    Template::render("login", context)
}

#[get("/register?<user_id>")]
async fn register(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
    let mut context = context!{
        user_logged_in: 0
    };

    if user_id.is_some() && user_id.unwrap().to_string() != 0.to_string(){
        let is_logged_in = api_check_logged_in(user_id.unwrap(), db).await.unwrap();
        if is_logged_in.to_string() == 1.to_string(){
            context.user_logged_in = user_id.unwrap();
        }
    }

    Template::render("register", context)
}

#[get("/post/<id>?<user_id>")]
async fn post(id: u16, user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
    let result = api_getpost(id, db).await.unwrap();
    let object: serde_json::Value = serde_json::from_str(&result).unwrap();

    let text = object["text"].as_str().unwrap();
    let paragraphs = text.lines().collect::<Vec<&str>>();

    let author = api_getuser(object["user_id"].to_string().parse::<i32>().unwrap(), db).await.unwrap();
    let author_object: serde_json::Value = serde_json::from_str(&author).unwrap();

    let mut context = context!{
        user_logged_in: 0,
        title: object["title"].as_str(),
        content: paragraphs,
        image: object["picture"].as_str(),
        author: author_object["full_name"].as_str()
    };

    if user_id.is_some() && user_id.unwrap().to_string() != 0.to_string(){
        let is_logged_in = api_check_logged_in(user_id.unwrap(), db).await.unwrap();
        if is_logged_in.to_string() == 1.to_string(){
            context.user_logged_in = user_id.unwrap();
        }
    }

    Template::render("post", context)
}

#[get("/addpost?<user_id>")]
async fn addpost(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
    let mut context = context!{
        user_logged_in: 0
    };

    if user_id.is_some() && user_id.unwrap().to_string() != 0.to_string(){
        let is_logged_in = api_check_logged_in(user_id.unwrap(), db).await.unwrap();
        if is_logged_in.to_string() == 1.to_string(){
            context.user_logged_in = user_id.unwrap();
        }
    }

    Template::render("addpost", context)
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(FromForm, Serialize, Deserialize)]
struct LoginForm {
    name: String,
    password: String
}

#[derive(FromForm, Serialize, Deserialize)]
struct AllPosts{
    text: String,
    id: i32,
    title: String,
}

#[derive(FromForm, Serialize, Deserialize)]
struct PostForm{
    text: String,
    picture: String,
    user_id: i32,
    title: String,
    image_name: String
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
async fn api_login(user_input: Form<LoginForm>, db: &State<DatabaseConnection>) -> Result<Redirect, String> {

    let response = User::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM \"user\" WHERE username = '{}'", user_input.name),
        [],
    )).all(db.inner()).await;

    match response {
        Ok(response) => {
            for column in response {
                if column.password == user_input.password {

                    let response = User::find_by_statement(Statement::from_sql_and_values(
                        DbBackend::Sqlite,
                        &format!("UPDATE \"user\" SET \"logged_in\" = 1 WHERE id = '{}'", column.id),
                        [],
                    )).all(db.inner()).await;

                    return Ok(Redirect::to(format!("/?user_id={}", column.id)))
                    // return format!("Correct password for user {}", column.username);
                }
                else {
                    return Err(format!("Incorrect password for user {}", column.username))
                }
            }
            return Err(format!("User {} not found", user_input.name))
        }
        Err(err) => {
            return Err(format!("Fetching error: {}", err))
        }
    }
}

//logging out
#[get("/logout/<user_id>")]
async fn api_logout(user_id: i32, db: &State<DatabaseConnection>) -> Result<Redirect, String> {

    println!("HERE");

    let response = User::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("UPDATE \"user\" SET \"logged_in\" = 0 WHERE id = '{}'", user_id),
        [],
    )).all(db.inner()).await;

    match response {
        Ok(response) => {
            return Ok(Redirect::to("/"))
        }
        Err(err) => {
            return Err(format!("Fetching error: {}", err))
        }
    }
}

//check_logged_in
#[get("/check_logged_in/<user_id>")]
async fn api_check_logged_in(user_id: i32, db: &State<DatabaseConnection>) -> Result<String, String> {

    let response = User::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM \"user\" WHERE id = '{}'", user_id),
        [],
    )).all(db.inner()).await;

    match response {
        Ok(response) => {
            return Ok(response[0].logged_in.to_string())
        }
        Err(err) => {
            return Err(format!("Fetching error: {}", err))
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

    // println!("{:?}", response);
    match response {
        Ok(response) => {
            for column in response {
                println!("Article '{}' fetched succesfully", column.title);
                let json_string = serde_json::to_string(&PostForm{
                    text: column.text,
                    picture: column.picture.unwrap_or_else(|| "".to_string()),
                    user_id: column.user_id,
                    title: column.title,
                    image_name: "".to_string()
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


//getting all posts
#[get("/getAllPosts")]
async fn api_getallposts( db: &State<DatabaseConnection>) -> Result<String, Status> {
    let response = Article::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM 'article' ORDER BY id DESC "),
        [],
    )).all(db.inner()).await;

    match response {
        Ok(posts) => {
            let mut post_forms = Vec::new();
            for post in posts {
                let post_form = AllPosts {
                    text: post.text,
                    title: post.title,
                    id: post.id
                };
                post_forms.push(post_form);
            }
            let json_string = serde_json::to_string(&post_forms).unwrap();
            return Ok(json_string);
        }
        Err(err) => {
            println!("Fetching error: {}", err);
            return Err(Status::BadRequest);
        }
    }
}


#[get("/getUser/<id>")]
async fn api_getuser(id: i32, db: &State<DatabaseConnection>) -> Result<String, Status> {
    let response = User::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("SELECT * FROM \"user\" WHERE id = '{}'", id),
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
            println!("User with id {} not found", id);
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
async fn api_register(user_input: Form<UserForm>, db: &State<DatabaseConnection>) -> Result<Redirect, Status> {
    format!("Your value: name - {}, email - {}, password - {}, password again - {}", user_input.name, user_input.email, user_input.password, user_input.password_again);

    if user_input.password == user_input.password_again {
        let response = User::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            &format!("INSERT INTO \"user\" (username, full_name, email, password, logged_in) VALUES ('{}', '{}', '{}', '{}', 0)", user_input.name,user_input.full_name,user_input.email,user_input.password),
            [],
        )).all(db.inner()).await;
        println!("{:?}", response);
        match response {
            Ok(response) => {
                println!("User {} created.", user_input.name);
                // return Status::Created;
                return Ok(Redirect::to("/"))
            }
            Err(err) => {
                println!("Error creating user: {}", err);
                return Err(Status::BadRequest)
            }
        }
    }
    else {
        println!("Passwords do not match");
        return Err(Status::ExpectationFailed)
    }
}


#[post("/addpost", data = "<user_input>")]
async fn api_addpost(user_input: Form<PostForm>, db: &State<DatabaseConnection>) -> Result<Redirect, Status> {
    let response = Article::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &format!("INSERT INTO \"article\" (text, picture, user_id, title) VALUES ('{}', '{}', '{}', '{}')", user_input.text,user_input.picture,user_input.user_id,user_input.title),
        [],
    )).all(db.inner()).await;

    println!("{:?}", response);
    match response {
        Ok(response) => {
            println!("Article {} created", user_input.title);
            return Ok(Redirect::to(format!("/?user_id={}", user_input.user_id)))
        }
        Err(err) => {
            println!("Error creating article: {}", err);
            return Err(Status::BadRequest)
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
        .mount("/", routes![index, login, register, post, addpost])
        .mount("/api", routes![api_getallposts,api_getuser,api_getpost,api_login, api_register, api_addpost, api_logout, api_check_logged_in])
}
