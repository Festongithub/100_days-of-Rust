fn add(a: u32) -> u32 {
    let sum = |a: u32|  a + a;
    return sum(a);

}

fn main() {

    let mut list = vec![34, 56, 76, 32, 21];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    
    println!("Before calling closure: {list:?}");
    
    only_borrows();
    
    println!("After calling closure: {list:?}");

    let mut borrows_mutable = || list.push(78);
    borrows_mutable();

    println!("After calling closure: {list:?}");

    let num = |x: u32| x + x;

    let a = num(8) + num(89);
    let b = num(80) + num(17);

    println!("Value closure of a is {} + {} = {}", a, b, a+b);

    let res = add(a);
    println!("Sum is {}", res);
}
