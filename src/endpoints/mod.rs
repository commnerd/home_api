use rocket::{Rocket, Build};

mod index;
mod web_hooks;

pub fn bootstrap() -> Rocket<Build>
{
    let rkt = rocket::build()
        .mount("/", routes![self::index::index]);

    web_hooks::bootstrap(Box::new(rkt))
}