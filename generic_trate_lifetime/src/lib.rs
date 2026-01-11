use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content; String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub usrname: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.usrname, self.content)
    }

    fn summarize_author(&self) -> Stirng {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
    println!("Breaking news1! {}", item1.summarize());
    println!("Breaking news2! {}", item2.summarize());
}

fn some_function<T, U>(t; &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Uceburgh"),
            content: String::from("The Pittsburgh Penguins once agin are the best hockey team in th NHL."),
        }
    } else {
        Tweet {
             username: String::from("horse_ebboks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
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
