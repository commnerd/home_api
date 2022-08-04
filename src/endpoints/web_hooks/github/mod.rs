use rocket::{Rocket, Build, routes};

mod ep;

pub fn bootstrap(rkt: Box<Rocket<Build>>) -> Rocket<Build>
{
    rkt.mount("/test", routes![ep::ep])
}