pub trait Summary {
    fn summarize(&self) -> String; // every type provides its own behavior for the method
    fn summarize_author(&self) -> String;
    // there can be more methods

    // or we can define a default method
    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn default_summarize_use_override_func(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // to use the default method, write nothing
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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/*pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}*/

pub fn notify<T: Summary>(item :&T) {
    println!("Breaking news! {}", item.summarize());
}

// for more parameters, impl allow different types but bound force them to have the same type
/*
pub fn notify(item1: &impl Summary, item2: &impl Summary)
pub fn notify<T: Summary>(item1: &T, item2: &T)
*/

// multiple trait
/*
pub fn notify(item: &(impl Summary + Display))
pub fn notify<T: Summary + Display>(item: &T)
*/

// clearer
/*
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
 */

// only return single type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // use the default method
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.default_summarize());

    // use method with default method
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.default_summarize_use_override_func());

    let return_tweet = returns_summarizable();
    println!("func return tweet, {}", return_tweet.default_summarize_use_override_func());

    // using traits to conditionally implement methods
    let integer_pair = Pair { x: 1, y: 2 };
    let new_pair = Pair::new(integer_pair.x, integer_pair.y);
    new_pair.cmp_display();
}