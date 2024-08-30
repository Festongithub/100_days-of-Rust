#!/usr/bin/rustc

struct Square {
    a: u32,
    b: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.a * self.b
    }
}

impl Square {
    fn perimeter(&self) -> u32 {
        (self.a + self.b) * 2
    }
}

impl Square {
    fn div(&self) -> u32 {
        self.a / self.b
    }
}

fn main() {
    let square = Square {
        a: 678,
        b: 908,
    };
    println!("The area is {}", square.area());
    println!("The quotient is {}", square.div());
    println!("The perimeter is {}", square.perimeter());
}