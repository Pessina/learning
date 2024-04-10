use std::fmt::Display;

pub trait Summarize {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarize for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I'm a news article")
    }
}

// impl Summarize for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} {}", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarize for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I'm a tweet")
    }
}

pub fn notify(content: &(impl Summarize + Display)) {
    println!("Breaking new from {}", content.summarize())
}

pub fn share<T, U>(content: &T, a: &U)
where
    T: Summarize + Display,
    U: Display,
{
    println!("Share our new content {}", content.summarize());
    println!("Share our new content {}", a)
}
