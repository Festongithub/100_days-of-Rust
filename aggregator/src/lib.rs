#!/usr/bin/rustc

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
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
        format!("{}: {}", self.username, self.content)
    }
}

pub struct User {
    pub name: String,
    pub email: String,
    pub views: String,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.email)
    }
}


fn main()
{
    let tweet = Tweet {
        username: String::from("macos_ebay"),
        content: String::from("we all rise from the ashes"),
        reply:false,
        retweet: false,
    };

    let user = User {
        name: String::from("Macron"),
        email: String::from("macron@email.com"),
        views: String::from("Good"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("new user: {}", user.summarize());
}
