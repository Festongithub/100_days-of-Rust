#!/usr/bin/rustc
mod lib;
use lib::{Summary, Tweet, User};

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

    println!("1 new tweet: {}", tweet.summarize);
    println!("new user: {}", user.summarize);
}
