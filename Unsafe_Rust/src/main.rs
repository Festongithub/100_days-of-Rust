unsafe fn add(a: i32, b:i32) -> i32 {
    return a + b;
}



fn main() {
    let mut num = 90;
    let _r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("Value is {:#?}", r2);

    unsafe {
        println!("value is {:#?}", *r2);
        println!("Address is {:#?}", r2);
    }

    let mut s1 = String::from("Address is new");
    let s2 = &mut s1 as *mut String;

    let i = 90;
    let j = 89;

    unsafe {
        println!("Address of s1 :{:#?}", s2);
        println!("Sum is {}", add(i,j));

    }

    println!("Message is {}", s1);
}
