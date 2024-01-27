use std::fs::File;
fn main() {

    let mut nums = Vec::new();
    let a = [67, 45, 68, 90, 44];

    nums.push(a[4]);
    nums.push(a[3]);
    nums.push(a[2]);
    nums.push(a[1]);
    nums.push(a[0]);

    for i in nums{
        println!("{i}");
    }

    let get_file_result = File::open("hello.txt");

    let get_file_result = match get_file_result{
        Ok(file) => file,
        Err(error) => {
            println!("File found");
            panic!("Problem opening file: {:?}", error);
        }
    };
}
