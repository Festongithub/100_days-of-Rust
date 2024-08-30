#!/usr/bin/rustc

struct Vendor {
    name: String,
    product: String,
    email: String,
    contact: u128,
}

fn Vendor_details(name: String, email: String) {

fn main() {
    let mut Vendo1 = Vendor {
        name: String::from("James Munroe"),
        product: String::from("Oranges"),
        email: String::from("JamesMunroe@gmail.com"),
        contact: 254743279637,
    };

    println!("{}", Vendo1.contact);
}
