fn main() {
    let x = 89;
    match x {
        89 => println!("Good one"),
        88 => println!("too bad"),
        44 => println!("testin waters"),
        _ => println!("anything"),
    }


    let i = Some(67);
    let j = 10;

    match i {
        Some(50) => println!("Got 50"),
        Some(i) => println!("Matched , i = {i}"),
        _=> println!("Default case, x = {x:?}"),
    }

    println!("at the end: i = {i:?}, j = {j}");

    let k = 100;

    match k {
        1..=100 => println!("Got new 100"),  
        100 | 10  => println!("It is a 100"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point {x : 89, y: 90};

    match p {
        Point {x, y: 90} => println!("On the x  axis at {x}"),
        Point {x: 89, y } => println!("On the y axis at {y}"),
        Point {x, y } => {
            println!("On neither axis: ({x} , {y})");
        }
    }
    enum Artist {
        Chrisbrown,
        Davido,
        Yen,
        Platinumz,
    }

    enum Music {
        Hiphop(Artist),
        Afrotrance(Artist),
        Asianbeats(Artist),
        Bongo(Artist),
    }
    let music = Music::Bongo(Artist::Platinumz);

    match music {
        Music::Hiphop(Artist::Chrisbrown) => {
            println!("Way too loud");
        },

        Music::Afrotrance(Artist::Davido) => {
            println!("Too proud");
        },
        Music::Asianbeats(Artist::Yen) => {
            println!("Too cozy");
        },
        Music::Bongo(Artist::Platinumz) => {
            println!("Kigoma near Tanganyika");
        },

        _ => (),
    }


}
