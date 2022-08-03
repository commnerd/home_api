use rocket::{Rocket, Build, routes};

pub fn init(bld: Rocket<Build>) -> Rocket<Build>
{
    bld.mount("/", routes![crate::index::index])
}