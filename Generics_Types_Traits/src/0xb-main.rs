fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}


struct Point<T> {
    x: T,
    y: T,
}

struct Revenue<T,U> {
    price:T,
    quantity: U
}

impl<T, U>Revenue<T, U> {
    fn reve(&self) -> (&T, &U) {
        T * U 
    }
}

fn main() {

    let _int = Point{ x: 5, y: 10};
    let float = Point{ x: 67.89, y:908.45};
    println!("The flaot points include {} {}", float.x, float.y);

    let n = Revenue{ price:7800, quantity: 5678};
    let rev = n.price * n.quantity;

    println!("Total revenue is: {}", n.reve());

    println!("Total revenue is : {:?}",rev);

    let numbers = vec![345, 678, 321, 900];

    let res = largest(&numbers);
    println!("The largest number is {res}");

}
