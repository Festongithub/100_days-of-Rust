macro_rules! say_hello {
    () => {
        println!("Hello")
    };
}

macro_rules! username {
    ($user: ident) => {
        fn $user(name: String) -> String {
            println!("The name is {}", name);
            name
        }
        
    };
}

macro_rules! calc {
    ($sum: ident) => {
        fn $sum(a: u32) -> u32 {
            println!("sum is {}", a + a);
            a + a
        }
    };

    ($mult: ident) => {
        fn $mult(a: i32, b: i32) -> i32 {
            println!("product is :{}", a * b);
            a * b
        }
    };


}

//Overload 

macro_rules! test {
    ($left:expr; and $right: expr) => {
        println!("{:?} and {:?} is {:?}", 
    stringify!($left), stringify!(right),
    $left && $right)        
    };

    ($left:expr; or $right: expr) => {
        println!("{:?} or {:?} is {:?}", 
    stringify!($left), stringify!(right),
    $left || $right)        
    };
}

calc!(sum);
calc!(mult);


fn main (){

    let name = String::from("Janet");
    username!(foo);
    foo(name);
    say_hello!();

    let i = sum(90);
    let j = mult(1,9);

    println!("The sum is {}", i);
    println!("The Product is {}", j);

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}