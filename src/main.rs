#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod customer;
use customer::{Customer};
use rocket_contrib::json::{Json, JsonValue};

#[get("/customers")]
fn get_customers() -> Json<JsonValue>  {
    Json(json!([
        "customer 1",
        "customer 2"
    ]))
}

#[post("/customers", data = "<customer>")]
fn post_customers(customer: Json<Customer>) -> Json<Customer> {
    customer
}

#[get("/customers/<customer_id>")]
fn get_customer(customer_id: String) -> Json<JsonValue> {
    Json(json!({"customer_id": customer_id}))
}

fn main() {
    rocket::ignite().mount("/", routes![
    get_customers, post_customers, get_customer]).launch();
}
