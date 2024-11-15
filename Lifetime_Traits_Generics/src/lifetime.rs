fn longest<'a >(x: & 'a str, y: & 'a str) -> & 'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main(){
    let x = 90; // x is only valid within this scope 

    let i = &x; // x is invalid because it is outside the scope

    println!("i : {i}");

    let s1 = String::from("abcdefr");
    let s2 = "xyz";

    let res = longest(s1.as_str(), s2);
    println!("Longest string is {res}");

    let novel = String::from("Call me Ishmael. Some years ageo...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };

}
