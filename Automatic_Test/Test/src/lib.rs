pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn  mul(a: u32, b: u32) -> u32 {
    a * b
}

//b fn message(name: str) -> String {
    //rmat!("Hello {name}!")
///

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess the value must be between 1 and 100, got {}.", value);
        }
        Guess {value}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn product() {
        let p = mul(90, 80);
        assert_ne!(p, 8100);
        assert_eq!(p, 7200);
        assert_ne!(p, 3456);
    }
    #[test]
    #[should_panic]
    fn greater_than_100()
    {
        Guess::new(200);
    }
}
