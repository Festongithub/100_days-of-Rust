fn some_string(name: &String) -> &String {
    println!("{name}");
    name
}

fn main() {

    let s = String::from("Hello");
    some_string(&s);
}
