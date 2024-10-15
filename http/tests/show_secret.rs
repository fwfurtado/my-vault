#[cfg(test)]
mod integration_tests {
    use http::make_rocket_server;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::Deserialize;

    #[derive(Deserialize)]
    struct ResponseBody {
        url: String,
        value: String,
    }

    #[test]
    fn should_show_existing_secret_data() {
        let client = Client::tracked(make_rocket_server()).expect("valid rocket client");

        let response = client
            .post("/users/signup")
            .header(ContentType::JSON)
            .body(r#"{"username": "user"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(
            response.into_string(),
            Some(r#"{"id":1,"username":"user"}"#.to_string())
        );

        let response = client
            .post("/secrets")
            .header(ContentType::JSON)
            .body(r#"{"name": "My secret", "url": "http://localhost", "secret": "confidential-secret"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.into_string(), Some(r#"{"id":1}"#.to_string()));

        let response = client.get("/secrets?url=http://localhost").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body = response
            .into_json::<ResponseBody>()
            .expect("valid response body");

        assert_eq!(body.url, "http://localhost");
        assert!(body.value.ends_with("confidential-secret"));
    }

    #[test]
    fn should_return_not_found_when_secret_does_not_exist() {
        let client = Client::tracked(make_rocket_server()).expect("valid rocket client");

        let response = client.get("/secrets?url=http://localhost").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
