macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name (){
            println!("You called {:?} ()", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! numbers {
    ($add: ident) => {
        println!("sum is  = {}", $add + $add)    
    };
    ($mult: ident) => {
        println!("sum is  = {}", $mult * $mult);
    }

}

fn main(){
    create_function!(foo);
    create_function!(bar);

    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x  + 2 * x - 1 
    });

    let add = 5;
    numbers!(add);

    let mult = 90;
    numbers!(mult);
}