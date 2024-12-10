#[derive(PartialEq, Debug)]
struct Response(usize);
macro_rules! handler {
    ($i: ident, $body: block) => {
        fn $i () -> Response $body
    }
}


macro_rules! calc {
    (addition) => { 
        1 + 1
    };
    (subtraction) => {
        90 - 10
    };

    (multiplication) => {
        80 * 10
    };
    (division) => {
        72 / 8
    };
}

macro_rules! repeat_n_times {
    ($n: expr, $text: expr) => {
        (0..$n).map(|_| format!("{}", $text)).collect::<Vec<String>>()
    };
}


fn main() {
    assert_eq!(2, calc!(addition));
    assert_eq!(80,calc!(subtraction));
    assert_eq!(800, calc!(multiplication));
    assert_eq!(9, calc!(division));
    assert_ne!(4 ,calc!(addition));
    assert_eq!(vec!["Hello", "Hello", "Hello"], repeat_n_times!(3, "Hello"));

    handler!(status_handler, {Response(200)});
    assert_eq!(status_handler(), Response(200));

}
