use jsonwebtoken::Header;
use rocket::{
    form::Form,
    fs::TempFile,
    http::{Cookie, CookieJar},
    serde::json::Json,
    Route, State,
};

use crate::{database::DatabaseHandler, models::user::User};

use self::token::{Claims, ENCODING_KEY};

pub mod debug;
pub mod token;

/// A form to get a username and password.
/// Used to register or login a user.
#[derive(FromForm)]
struct UserForm {
    username: String,
    password: String,
}

/// Takes an old and new password, to change the password of a user.
#[derive(FromForm)]
struct ChangePasswordForm {
    old_password: String,
    new_password: String,
}

/// Gets the data needed to create a new post.
#[derive(FromForm)]
struct CreatePostForm<'a> {
    content: Option<String>,
    image: Option<TempFile<'a>>,
}

#[post("/auth/register", data = "<user>")]
fn auth_register(db: &State<DatabaseHandler>, user: Form<UserForm>) -> Result<String, String> {
    let username = &user.username;
    let password = &user.password;

    let created_user = User::create(username.to_owned(), password.to_owned())?;

    let user_in_db = db.find_user_by_name(username)?;
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
fn auth_login(
    db: &State<DatabaseHandler>,
    cookies: &CookieJar<'_>,
    user: Form<UserForm>,
) -> Result<Json<User>, String> {
    let username = &user.username;
    let password = &user.password;

    let user = db.login_user(username, password)?;

    let claim = Claims {
        exp: 0,
        sub: user.name,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claim, &ENCODING_KEY)
        .map_err(|err| err.to_string())?;

    cookies.add(
        Cookie::build("api-token", token)
            .path("/api")
            .secure(true)
            .http_only(true)
            .finish(),
    );

    Ok(Json(user))
}

#[post("/auth/change-password", data = "<change_pass>")]
fn auth_change_pass(
    db: &State<DatabaseHandler>,
    change_pass: Form<ChangePasswordForm>,
) -> Result<(), String> {
    let old_password = &change_pass.old_password;
    let new_password = &change_pass.new_password;
    let username = "Placeholder";

    let is_old_password_is_correct = db.login_user(username, old_password).is_ok();
    if !is_old_password_is_correct {
        return Err("Old password is incorrect!".to_string());
    }

    let hashed_password = User::hash_password(new_password)?;

    db.change_password(username, &hashed_password)?;
    //                                            ^--- This returns error if any, otherwise continues running.
    // Having reached this point means that everything went correctly.
    Ok(())
}

#[post("/create-post", data = "<post>")]
fn create_post(db: &State<DatabaseHandler>, post: Form<CreatePostForm<'_>>) {}

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
