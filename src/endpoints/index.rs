#[get("/")]
pub fn index() -> &'static str {
    "Hello World!"
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello World!".to_string());
    }
}