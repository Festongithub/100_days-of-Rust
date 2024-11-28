#[link(name = "add")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        println!("sum :{}", add(90, 90));
    }
}
