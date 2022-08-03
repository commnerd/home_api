#[macro_use] extern crate rocket;

use rocket::launch;

mod index;
mod endpoints;

#[launch]
fn rocket() -> _ {
    crate::endpoints::init(rocket::build())
}