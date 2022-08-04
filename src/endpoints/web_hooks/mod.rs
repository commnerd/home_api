use rocket::{Rocket, Build};

mod github;

pub fn bootstrap(rkt: Box<Rocket<Build>>) -> Rocket<Build>
{
    github::bootstrap(rkt)
}