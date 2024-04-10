pub mod media;

use media::{notify, share, NewsArticle, Tweet};

use crate::media::Summarize;

fn main() {
    let news_article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: true,
    };

    println!("{}", tweet.summarize());
    println!("{}", news_article.summarize());

    notify(&tweet);
    share(&news_article, &news_article);
}
