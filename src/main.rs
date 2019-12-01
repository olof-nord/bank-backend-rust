#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/customers")]
fn get_customers() -> &'static str {
    "List all customers"
}

#[post("/customers")]
fn post_customers() -> &'static str {
    "Create a new customer"
}

fn main() {
    rocket::ignite().mount("/", routes![
    get_customers, post_customers]).launch();
}
