use http::make_rocket_server;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;


#[test]
fn should_post_username_and_retrieve_user_json() {
    let client = Client::tracked(make_rocket_server()).expect("valid rocket client");

    let response = client.post("/users/signup")
        .header(ContentType::JSON)
        .body(r#"{"username": "user"}"#)
        .dispatch();

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some(r#"{"id":0,"username":"user"}"#.to_string()));
}

#[test]
fn should_return_conflict_when_try_to_create_user_twice() {
    let client = Client::tracked(make_rocket_server()).expect("valid rocket client");

    let response = client.post("/users/signup")
        .header(ContentType::JSON)
        .body(r#"{"username": "user"}"#)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let response = client.post("/users/signup")
        .header(ContentType::JSON)
        .body(r#"{"username": "user"}"#)
        .dispatch();

    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(response.into_string(), Some(r#"User already exists"#.to_string()));
}