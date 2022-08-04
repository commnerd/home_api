#[macro_use] extern crate rocket;

use rocket::launch;

mod endpoints;

#[launch]
fn rocket() -> _ {
    crate::endpoints::bootstrap()
}