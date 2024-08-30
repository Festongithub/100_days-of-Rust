struct User {
    name: String,
    age: u64,
    course: String,
}

fn user_details(user: &User) -> String {
    format!("{} is enrolled in the {} course", user.name, user.course)
}

fn main() {
    let user1 = User {
        name: String::from("Fanice Jelly"),
        age: 89,
        course: String::from("Industrialization"),
    };
    println!("User details include: {}", user_details(&user1));
}

