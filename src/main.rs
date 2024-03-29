use database::DatabaseHandler;
use rocket::{fs::FileServer, response::Redirect};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;
mod api;
mod app;
mod database;
mod models;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/app"))
}

#[launch]
fn rocket() -> _ {
    let database_handler = match DatabaseHandler::create_connection() {
        Ok(handler) => handler,
        Err(e) => panic!("{}", e),
    };
    rocket::build()
        .mount("/", routes![index])
        .mount("/app", app::get_app_routes())
        .mount("/api", api::get_api_routes())
        .mount("/debug", api::debug::get_debug_routes())
        .mount("/static", FileServer::from("./static"))
        .manage(database_handler)
        .attach(Template::fairing())
}
