
fn username(name: String) -> String {
    println!("username is {}", &name);
    name
}


fn main() {
    let i = String::from("Janet");
    username(i);

    let b = Box::new(890);
    println!("b = {b}");

    let x = 56;
    let y = Box::new(x);

    //println!(" address of y : {:p}", y);
    //println!(" value of y : {}", y);

    assert_eq!(56, x);
    assert_eq!(56, y);
}
