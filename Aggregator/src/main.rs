
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summarize{
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


fn main() {
    let article = NewsArticle {
    headline: String::from("Penguin from madagascar"),
    location: String::from("Madagascar"),
    author: String::from("Niarobi"),
    content: String::from("Build for the game"),
    };

    println!("New article available! {}", article.summarize());
}
