use rocket::{Rocket, Build};

mod ep;
mod push;

pub fn bootstrap(mut rkt: Box<Rocket<Build>>) -> Rocket<Build>
{
    endpoint!(rkt, "/test", ep::ep);
    endpoint!(rkt, "/push", push::push);
    *rkt
}