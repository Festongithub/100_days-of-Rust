fn main() {
    let x = 67;
    let add_x = |y| y + x;

    let mult_x = |a, z| a * z;

    println!("Product is: {}", mult_x(45, 67));
    println!("Result is: {}", add_x(90));


    let mut names = vec!["Peter", "Sheila", "Johnny"];
    println!("Before defining closure: {names:?}");

    let mut users = || names.push("Janet");
    users();
    println!("After calling closure: {names:?}");


    let v1 = vec![1, 2,3, 4,5];
    let v1_iter = v1.iter();
    for i in v1_iter {
        println!("Got: {i}");
    }

    let total: i32 = v1.iter().sum();
    println!("Sum of the values is {total}");

    let bills: Vec<i32> = vec![900, 543, 321, 567];
    let v2 : Vec<_> = bills.iter().map(|x| x * 2).collect();
    assert_eq!(v2, vec![1800, 1086, 642, 1134]);


    let b = Box::new(78);
    println!("b = {b}");


}
