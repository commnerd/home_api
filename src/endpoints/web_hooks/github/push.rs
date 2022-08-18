#[get("/push")]
pub fn push() -> &'static str {
    "Ok"
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn push() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Ok".to_string());
    }
}