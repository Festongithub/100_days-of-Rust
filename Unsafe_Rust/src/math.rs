#[link(name = "math")]

extern "C" {
    fn mult(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        let r = mult(89, 90);
        println!("Product :{}", r);
    }
}
