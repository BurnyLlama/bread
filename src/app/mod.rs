use rocket::Route;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn landing() -> Template {
    Template::render("landing", context! {})
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context! {})
}

#[get("/register")]
fn register() -> Template {
    Template::render("register", context! {})
}

#[get("/create-post")]
fn create_post() -> Template {
    Template::render(
        "app/create-post",
        context! {
            username: "FooBar",
            posts_left: 1,
            posts_per_day: 2,
        },
    )
}

#[get("/friends")]
fn friends() -> Template {
    Template::render("app/friends", context! {})
}

#[get("/random")]
fn random() -> Template {
    Template::render(
        "app/random",
        context! {
            posts_left: 3,
            posts_per_day: 10,
        },
    )
}

#[get("/profile")]
fn profile() -> Template {
    Template::render("app/profile", context! { username: "FooBar" })
}

pub fn get_app_routes() -> Vec<Route> {
    routes![
        landing,
        register,
        login,
        create_post,
        friends,
        search,
        random,
        profile
    ]
}
