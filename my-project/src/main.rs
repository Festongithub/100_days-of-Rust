mod garden;
use garden::{add, sub,mult, Rectangle, Student};

fn main() {
    let sum = add(89, 90);
    let d = sub(80, 10);
    let q = mult(89, 89);

    println!("Sum is {sum}");
    println!("difference is {d}");
    println!("difference is {q}");
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 78,
        height: 90,
    };

    println!("The ares of the rectangle is :{}", rect1.area());
    println!("The ares of the rectangle is :{}", rect1.perimeter());

    let student1 = Student {
        name:String::from("Jelly Roller"),
        age: 45,
        course: String::from("Bachelor of Arts"),
    };

    println!("Name is {}", student1.get_name());
    println!("Age is {}", student1.get_age());
    println!("Course is {}", student1.course_name());
}
