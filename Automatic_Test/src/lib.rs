use std::fmt::format;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub struct Guess {
    value : i32
}


impl Guess {
    pub fn new(value: i32 ) -> Guess {
        if value < 1
        {
            panic!("Guess value must be between 1 and 100, got {value}");
        } else if value > 100 {
            panic! (
                "Guess value must be less than or equal to 100, got {value}"
            )
        }
        Guess {value}
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}


#[derive(Debug)]
struct Product {
    price: u64,
    quanity: u64,
}

impl Product {
    fn revenue(&self) -> u64 {
        self.price * self.quanity
    }

    fn price(&self) -> u64 {
        return self.price
    }
}

pub fn greetings(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn mult(a: u32, b: u32) -> u32 {
    a * b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_mult() -> Result<(), String> {
        let product = mult(89, 90);

        if product == 8010 {
            Ok(())
        } else {
            Err(String::from("product of eighty nine and ninety is eight hundred and ten"))
        }
    }

    #[test]

    fn name_test() -> Result<(), String>{
        let name = greetings("Carol");

        if name == "Hello Carol!"{
            Ok(())
        } else {
            Err(String::from("Should be Hello Carol"))
        }
    }
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller(){
        let l = Rectangle {
            width: 8,
            height: 7
        };
        let s = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(l.can_hold(&s));
    }

    #[test]
    fn test_area(){
        let a = Rectangle {
            width: 90,
            height: 90
        };
        let area = a.area();
        assert_eq!(area, 8100);
    }

    #[test]
    fn test_revenue() {
        let p = Product {
            price: 9000,
            quanity: 10000,
        };
        let rev = p.revenue();
        assert_ne!(rev, 9000);

        let rice = p.price();
        assert_eq!(rice, 9000)
    }

    #[test]
    fn test_greetings(){
        let res = greetings("Carol");
        assert!(res.contains("Carol"),
        "Greetinsg did not contain name, value was {res}"
    )
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn test_greater_than_100(){
        Guess::new(200);
    }
}
