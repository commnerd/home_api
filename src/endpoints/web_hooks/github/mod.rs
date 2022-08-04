use rocket::{Rocket, Build, routes};

mod ep;

pub fn bootstrap(rkt: Box<Rocket<Build>>) -> Rocket<Build>
{
    endpoint!(rkt, "/test", routes![ep::ep])
}