
fn coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn location((name, venue): &(String, String)){
    println!("Currentl at {name} at {venue}");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn call_add(u: i32, v: i32) -> i32 {
    add(u, v)
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color{
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    let point = (89, 67);
    coordinates(&point);

    let (place, venue) = (String::from("New York"), String::from("Thomson Avenue"));

    location(&(place, venue));

    let u = 98;
    let v = 98;

    println!("{}", call_add(u, v));
}
