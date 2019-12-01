#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/customers")]
fn get_customers() -> String {
    format!("List all customers")
}

#[post("/customers")]
fn post_customers() -> String {
    format!("Create a new customer")
}

#[get("/customers/<customer_id>")]
fn get_customer(customer_id: String) -> String {
    format!("Get customer with id {}", customer_id)
}

fn main() {
    rocket::ignite().mount("/", routes![
    get_customers, post_customers, get_customer]).launch();
}
