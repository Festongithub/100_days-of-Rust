fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn revenue(y: Option<u32>) -> Option<u32> {
    match y {
        None => None,
        Some(u) => Some(u * u),
    }
}


fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let x = 98;
    let seven = revenue(Some(x));

    println!("{:?}", seven);
    println!("{:?}", six);
}
