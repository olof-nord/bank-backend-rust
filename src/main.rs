#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

mod customer;
use customer::{Customer};
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

#[get("/customers", format = "json")]
fn get_customers() -> JsonValue  {
    json!([
        { "id": Uuid::new_v4() },
        { "id": Uuid::new_v4() }
    ])
}

#[post("/customers", data = "<customer>", format = "json")]
fn post_customer(mut customer: Json<Customer>) -> Json<Customer> {
    customer.id = Option::from(Uuid::new_v4());

    customer
}

#[get("/customers/<customer_id>", format = "json")]
fn get_customer(customer_id: rocket_contrib::uuid::Uuid) -> JsonValue {
    json!({
        "id": format!("{}", customer_id),
        "first_name": "TBD",
        "last_name": "TBD"
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
    get_customers, post_customer, get_customer])
}

fn main() {
    rocket().launch();
}
