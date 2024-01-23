fn main() {
    println!("Hello, world!");

    let mut  v: Vec<i32> = Vec::new();
    let a = [90, 54, 34, 67];

    v.push(a[0]);
    v.push(a[1]);
    v.push(a[2]);
    v.push(a[3]);

    // using the index method to access the vector values
    let third = &v[3] + &v[2];

    println!("Third element is {}",third);

    for ii in &v {
        println!("{}", ii);
    }
    let mut nums: Vec<i32> = Vec::new();

    let b = a;
    nums.push(b[0]);
    nums.push(b[1]);
    nums.push(b[2]);
    nums.push(b[3]);

    let one = &nums[0];
    println!("{} is first in element", one);


    let mut gig = vec![8900, 6788, 3423, 6784, 9000];
    for j in &mut gig {
        *j += 8900;
        println!("{j}");
    }
}
