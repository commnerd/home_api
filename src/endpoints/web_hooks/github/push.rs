#[post("/", data = "<input>")]
pub fn push(input: String) -> String { // &'static str {
    input
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use std::fs;

    #[test]
    fn push() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let req = fs::read_to_string("src/endpoints/web_hooks/github/test/events/push.json");
        
        let resp = client.post("/push").body(req.as_ref().unwrap()).dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.into_string().unwrap(), req.unwrap());
    }
}