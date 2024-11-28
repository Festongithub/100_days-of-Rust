fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(12);
    v.push(20);

    for i in &v {
        println!("{i:?}");
    }
    v

}

fn print_padovan() {

    let s = vec!["Don".to_string(), "Kelvin".to_string(), "Brian".to_string()];
    let t = s.clone();
    let u = s;

    println!("{u:?}");

    let mut padovan = vec![90, 90, 90];
    for i in 3..10{
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}


fn user_detail<'a>(name: & 'a String, residence: & 'a String) -> (& 'a String, & 'a String) {
    println!("{name} live in {residence}!");
    (name, residence)
}


fn main() {

    let user = String::from("Janet");
    let home = String::from("New York");
    user_detail(&user, &home);

    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    print_padovan();
    build_vector();
    println!("Hello, world!");
}
