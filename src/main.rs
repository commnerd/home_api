#[macro_use] extern crate rocket;
#[macro_use] mod macros;

use rocket::launch;

mod endpoints;

#[launch]
fn rocket() -> _ {
    crate::endpoints::bootstrap()
}