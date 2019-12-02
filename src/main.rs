#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod customer;
use customer::{Customer};
use rocket_contrib::json::{Json, JsonValue};

#[get("/customers", format = "json")]
fn get_customers() -> JsonValue  {
    json!([
        { "customer_id": "1" },
        { "customer_id": "2" }
    ])
}

#[post("/customers", data = "<customer>", format = "json")]
fn post_customers(customer: Json<Customer>) -> Json<Customer> {
    customer
}

#[get("/customers/<customer_id>", format = "json")]
fn get_customer(customer_id: String) -> JsonValue {
    json!({ "customer_id": customer_id })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
    get_customers, post_customers, get_customer])
}

fn main() {
    rocket().launch();
}
