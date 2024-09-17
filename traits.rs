use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Summary_Two {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

// default implementation
impl Summary_Two for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// Syntax sugar for the trait bound.
pub fn notify(item: &impl Summary) {
    println!("BREAKING NEWS! {}", item.summarize());
}

// This is the trait bound version
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("BREAKING NEWS! {}", item.summarize());
}

// two arguments:
pub fn notify_dual_args(item1: &impl Summary, item2: &impl Summary) {
    println!("BREAKING NEWS! {} {}", item1.summarize(), item2.summarize());
}

// forcing the same type using trait bound
pub fn notify_dual_args_trait_bound<T: Summary>(item1: &T, item2: &T) {
    println!("BREAKING NEWS! {} {}", item1.summarize(), item2.summarize());
}

// Multiple traits
pub fn notify_multi_traits(item: &(impl Summary + Display)) {
    println!("BREAKING NEWS! {}", item.summarize());
}

// Trait version
pub fn notify_multi_traits_bound<T: Summary + Display>(item: &T) {
    println!("BREAKING NEWS! {}", item.summarize());
}


// Trait bounds getting messy
fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    55
}

// Using a where clause to clean it up
fn some_func_where_clause<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
    {
        55
}

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse ebooks"),
        content: String::from("of course, horses"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}


// conditional implementation 
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Display trait which immplements the ToString trait
// impl <T: Display> ToString for T {
//     // --snip--
// } 

// let s = 3.to_string(); <== this uses the above ToString 


fn main() {
    let tweet = Tweet {
        username: String::from("torgian"),
        content: String::from("of course you would say that, you jerkwhad",),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburge, PA, USA"),
        author: String::from("torgian"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
            hockey team in the NHL.",),
    };

    println!("New article available! {}", article.summarize());
}

