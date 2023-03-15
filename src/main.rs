use api::debug::get_debug_routes;

#[macro_use]
extern crate rocket;
mod api;
mod database;
mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/debug", get_debug_routes())
}
