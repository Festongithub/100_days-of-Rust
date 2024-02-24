pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn sub(a: usize, b: usize) -> usize {
a - b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        //This function returns a boolean(true or false 
        //for the condition expressed
        self.width > other.width && self.height > other.height}
}


pub fn mul(a: u32) -> u32 {
    a * 2
}

//checking for string 

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn list_names(user: &str) -> String {
    String::from("Hello {user}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(128, mul(64));
        assert_ne!(124, mul(32));
    }
    #[test]
    fn user()
    {
        let u = list_names("William");
        assert!(u.String::from("Hello William"));
    }
    #[test]
    fn greeting_contains_name() {
        let r = greeting("Carol");
        assert!(r.contains("carol"));
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(53, add(45, 9));
    }
    #[test]
    
    fn check_sub()
    {
    let diff = sub(561, 78);
    assert_eq!(diff, 483);
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
}
