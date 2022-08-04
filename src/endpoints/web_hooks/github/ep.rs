#[get("/")]
pub fn ep() -> &'static str {
    "Test"
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn ep() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/test").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Test".to_string());
    }
}