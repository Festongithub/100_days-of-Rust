
/*
 * Trait defines the functionality a particular type 
 * has and can share with other Types
 * 
 * The trait Summary is made public and can be used by other crates
 *
 *
 * */
pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub trait Book {
    fn details(&self) -> u32;
}


pub struct River {
    pub pages: u32,
    pub auth_number: u32,
    pub ticket: u32,
}

impl Book for River {
    fn details(&self) -> u32 {
        self.pages + self.auth_number + self.ticket
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("best books"),
        reply: false,
        retweet: false,
    };

    let book = River {
        pages: 678,
        auth_number: 80,
        ticket: 1234,
    };
    println!("1 new tweet : {}", tweet.summarize());
    println!("New book : {}", book.details());

    let article = NewsArticle {
        headline: String::from("ALX is building bigger and Better"),
        location: String::from("Nairobi, Kenya"),
        author: String::from("Faces of ALX"),
        content: String::from(
            "The ALX is building back better for the AFrican continent"
        )
    };

    println!("New Article available!{}", article.summarize());

}
