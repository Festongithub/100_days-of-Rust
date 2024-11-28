fn greetings(name: String) -> String {
    println!("Hello {name}!");
    name
}

fn user_id(f_name: &String, number: i32) -> (&String, i32) {
    println!("Hello {f_name} code {number}");
    (f_name,number)
}

fn add_text(text: &mut String) -> String {
    println!("{text}, welcome to home");
    text.to_string()
}


fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn build(director: &String) ->&String{
    println!("New work as {director}");
    director
}


fn phone_number(telephone: &String) -> usize {
    telephone.len()
}



fn main() {

    let mut message = String::from("Dear user");
    add_text(&mut message);


    let mut s = String::from("Hello");

    // mutable and immutable

    let text = String::from("Bring the word");
    s.push_str(&text);

    println!("message is {text}");
    println!("New message is {s}");

    let c = String::from('X');
    let i = &c;

    println!("{i}, Y");

    let username = String::from("Hendry");
    greetings(username);

    let cake = String::from("Jane");
    let nums = 8900;


    user_id(&cake, nums);

    let i = 900;
    let j = 876;

    println!("{}", add(i,j));

    let u = &i as *const u32;

    println!("The address of i is {:p}", u);

    let test = String::from("Video Director");

    build(&test);

    let p = String::from("24563893736373673763");
    println!("{}", phone_number(&p));
}
