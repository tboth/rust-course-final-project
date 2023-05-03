use rocket_dyn_templates::{Template, context};
use rocket::State;
use sea_orm::*;
use serde::{Serialize};
use crate::api_endpoints::*;

#[derive(Serialize)]
pub struct Post {
    title: String,
    excerpt: String,
    id: String
}

#[derive(Serialize)]
pub struct Posts {
    user_logged_in: i32,
    posts: Vec<Post>
}

#[get("/?<user_id>")]
pub async fn index(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {  
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
pub async fn login(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
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
pub async fn register(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
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
pub async fn post(id: u16, user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
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
pub async fn addpost(user_id: Option<i32>, db: &State<DatabaseConnection>) -> Template {
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