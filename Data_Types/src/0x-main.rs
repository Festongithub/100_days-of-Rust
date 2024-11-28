fn main() {
    let b = 89;
    let a = &b;
    println!("{}", a);

    let needle = 202 + 1 ;
    let heystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &heystack {
        if *item == needle{
            println!("{}", item);
        }
    }

    let numbers = vec![123, 454, 211, 56];
    let target = &numbers[0];

    println!("{target:?}");

    println!("{numbers:?}");

    let i = 90;
    let j = 100;
    println!("{}", add_with_lifetime(&i, &j));

    let username = String::from("Janet Ochieng");
    println!("{}", get_name(&username));
    println!("{}", add(i, j));
    println!("{}", mult(i, j));

}

fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn get_name<'a>(name: &'a String) -> &String{
    println!("Hello {name}!");
    name
}

fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T{
    i + j
}

fn mult<T: std::ops::Mul<Output = T>>(i: T, j: T) -> T {
    i * j
}

