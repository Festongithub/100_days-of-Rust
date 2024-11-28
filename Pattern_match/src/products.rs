enum Products {
    Oranges,
    Mangoes,
    Pineapple,
    Clothes
}

fn divisible_by_11(nums: Vec<i32>) -> Vec<i32> {
    let mut numbers = Vec::new();

    for num in &nums {
        if num % 11 == 0 {
            println!("{} is divisible by 11", num);
	    numbers.push(*num);
        } else {
            println!("{} is not divisible by 11", num);
        }
    }
    numbers
}
fn main() {
    let cloth = Products::Clothes;

    if let Products::Clothes = cloth {
        println!("Going right!");
    } else {
        println!("Not found");
    }

    let mut  nums = Vec::new();

    nums.push(120);
    nums.push(121);
    nums.push(242);

    divisible_by_11(nums);
}
