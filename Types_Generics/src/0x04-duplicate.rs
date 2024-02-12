#!/usr/bin/rustc
struct Point<T, U>
{
    x: T,
    y: U,
}

impl<T, U>Point<T,U> {
    fn takeup<T1, U1> (
        self,
        other: Point<T1, U1>,
    ) -> Point<T1, U> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main()
{
    let p = Point {x: 5, y: 4};
    let p1 = Point {x: 67, y:"Hello"};
    println!("p.x = {}", p.x());

    let p3 = p.takeup(p1);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let s_1 = Student{name:"Festus", age:78, number:89};
    println!("s_1.name: {}", s_1.id());
}
