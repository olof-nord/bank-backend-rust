use crate::rocket;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn get_customer_invalid_param() {
    let client = Client::new(rocket()).unwrap();
    let response = client
        .get("/customers/1")
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_customer_valid() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .get("/customers/894fd4d9-82b5-4d3e-9776-ee3851382880")
        .header(ContentType::JSON)
        .dispatch();

    let body = response.body().unwrap().into_string().unwrap();

    assert!(body.contains("id"));
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_customers() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .get("/customers")
        .header(ContentType::JSON)
        .dispatch();

    let response_body = response.body().unwrap().into_string().unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert!(response_body.contains("id"));
}

#[test]
fn post_customer_valid() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client
        .post("/customers")
        .body(r#"{
            "first_name": "Sven",
            "last_name": "Svensson",
            "email": "sven.svensson@gmail.com",
            "nationality": "Sweden"
        }"#)
        .header(ContentType::JSON)
        .dispatch();

    let response_body = response.body().unwrap().into_string().unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert!(response_body.contains("id"));
}

#[test]
fn post_customer_invalid_missing_parameters() {
    let client = Client::new(rocket()).unwrap();
    let response = client
        .post("/customers")
        .body(r#"{
            "first_name": "Sven",
            "last_name": "Svensson",
            "email": "sven.svensson@gmail.com"
        }"#)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);
}