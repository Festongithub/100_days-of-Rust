struct User {
    name: String,
    email: String,
    code_number: u128,
    registration_number: u128,
    hostel: String,
    room_number:u32,  
}

struct Product {
    product_name: String,
    quantity: i32,
    username: String,
    price: i32,
}

fn revenue(quantity: i32, price: i32) {
    println!("Total revenue is {}", quantity * price);
}
// The aspect of ownership is important in struct methods
fn details(name: &String, email: &String){
    println!(" name: {} and email {}", name, email);
}


fn main() {
    let Student = User {
        name: String::from("John wick"),
        email: String::from("John@gmail.com"),
        code_number: 7899898875576,
        registration_number: 898774397,
        hostel: String::from("Gandhi House"),
        room_number: 8976
    };

    println!("The student number is {}", Student.code_number);
    details(&Student.name, &Student.email);

    let Oranges = Product {
        product_name: String::from("Oranges"),
        quantity: 100,
        username: String::from("Janet Cheru"),
        price: 80,
    };
    revenue(Oranges.quantity, Oranges.price);
}
