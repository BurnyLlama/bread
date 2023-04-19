use rocket::{form::Form, fs::TempFile, serde::json::Json, Route, State};

use crate::{database::DatabaseHandler, models::user::User};

pub mod debug;

#[derive(FromForm)]
struct UserForm {
    username: String,
    password: String,
}

#[derive(FromForm)]
struct CreatePost<'a> {
    content: Option<String>,
    image: Option<TempFile<'a>>,
}

#[post("/auth/register", data = "<user>")]
fn auth_register(db: &State<DatabaseHandler>, user: Form<UserForm>) -> Result<String, String> {
    let username = &user.username;
    let password = &user.password;

    let created_user =
        User::create(username.to_owned(), password.to_owned()).map_err(|err| err.to_string())?;

    let user_in_db = db.find_user_by_name(&username)?;
    if user_in_db.is_some() {
        return Err("User already exists with that name!".to_string());
    }

    let user_id = db.save_user(&created_user)?;

    match user_id {
        Some(id) => Ok(id.to_string()),
        None => Err("Couldn't get ObjectId of the user!".to_string()),
    }
}

#[post("/auth/login", data = "<user>")]
fn auth_login(db: &State<DatabaseHandler>, user: Form<UserForm>) -> Result<Json<User>, String> {
    let username = &user.username;
    let password = &user.password;

    match db.find_user_by_name(username)? {
        Some(user) => {
            let hash_matches = argon2::verify_encoded(&user.password, password.as_bytes())
                .map_err(|err| err.to_string())?;

            if !hash_matches {
                return Err("Wrong password!".to_string());
            }

            Ok(Json(user))
        }
        None => return Err("User not found!".to_string()),
    }
}

#[post("/auth/change-password")]
fn auth_change_pass(db: &State<DatabaseHandler>) {}

#[post("/create-post", data = "<post>")]
fn create_post(db: &State<DatabaseHandler>, post: Form<CreatePost<'_>>) {}

#[post("/settings")]
fn settings(db: &State<DatabaseHandler>) {}

#[post("/follow/<user_id>")]
fn follow(db: &State<DatabaseHandler>, user_id: &str) {}

#[post("/unfollow/<user_id>")]
fn unfollow(db: &State<DatabaseHandler>, user_id: &str) {}

pub fn get_api_routes() -> Vec<Route> {
    routes![
        auth_register,
        auth_login,
        auth_change_pass,
        create_post,
        settings,
        follow,
        unfollow
    ]
}
