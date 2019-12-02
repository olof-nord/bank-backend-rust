#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod customer;
use customer::{Customer};
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

#[get("/customers", format = "json")]
fn get_customers() -> JsonValue  {
    json!([
        { "customer_id": Uuid::new_v4() },
        { "customer_id": Uuid::new_v4() }
    ])
}

#[post("/customers", data = "<customer>", format = "json")]
fn post_customers(mut customer: Json<Customer>) -> Json<Customer> {
    customer.id = Option::from(Uuid::new_v4());

    customer
}

#[get("/customers/<customer_id>", format = "json")]
fn get_customer(customer_id: rocket_contrib::uuid::Uuid) -> JsonValue {
    json!({
        "customer_id": format!("{}", customer_id)
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
    get_customers, post_customers, get_customer])
}

fn main() {
    rocket().launch();
}
