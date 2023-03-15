use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, Route, State};
use serde::Deserialize;

use crate::{
    database::DatabaseHandler,
    models::{post::Post, user::User},
};

#[get("/test-user")]
pub fn test_get_user() -> Json<User> {
    Json(User::create("Foo".to_string(), "Bar".to_string()))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateUser<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[post("/test-user", data = "<input>")]
pub fn test_post_user(
    input: Json<CreateUser<'_>>,
    db: &State<DatabaseHandler>,
) -> Result<Json<User>, String> {
    let user = User::create(input.username.to_string(), input.password.to_string());
    let user_id = db.save_user(&user)?;

    if user_id.is_none() {
        return Err("No such user!".to_string());
    }

    let user_from_db = db.find_user_by_id(user_id.unwrap())?;

    match user_from_db {
        Some(user) => Ok(Json(user)),
        None => Err("No such user!".to_string()),
    }
}

#[get("/test-post")]
pub fn test_get_post() -> Json<Post> {
    Json(Post::create(
        ObjectId::new(),
        Some("Lorem ipsum dolor sit amet consectetur adipis.".to_string()),
        None,
    ))
}

pub fn get_debug_routes() -> Vec<Route> {
    routes![test_get_user, test_post_user, test_get_post]
}
