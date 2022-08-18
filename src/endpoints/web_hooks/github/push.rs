use serde_json::{Result, Value};

#[post("/", data = "<input>")]
pub fn push(input: String) -> String {
    let data: Value = serde_json::from_str(&input).unwrap();
    let url: String = data["repository"]["clone_url"].to_string();
    url
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use std::fs;

    #[test]
    fn push_result() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let req = fs::read_to_string("src/endpoints/web_hooks/github/test/events/push.json");
        
        let resp = client.post("/push").body(req.as_ref().unwrap()).dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.into_string().unwrap(), serde_json::to_string("https://github.com/commnerd/maikr.git").unwrap());
    }
}