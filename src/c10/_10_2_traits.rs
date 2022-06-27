use std::any::Any;
use std::fmt::Display;

pub fn main() {



}

//定义trait
pub trait Summary {
    // trait默认实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//实现trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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

//trait作为参数 的三种写法。推荐使用第三种
// trait作为参数,使用impl
pub fn notify111(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// trait作为参数,使用 trait bound
pub fn notify222<T:Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// trait作为参数,使用 where
pub fn notify333<T>(item: T) where T:Summary {
    println!("Breaking news! {}", item.summarize());
}

// 使用+ 来表示实现多个bound
pub fn notifymulti<T>(item: T) where T:Summary+Display {
    println!("Breaking news! {}", item.summarize());
}

// trait作为返回值，这里无法编译，因为不能返回多个类型。
//fn returns_summarizable(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//            headline: String::from("Penguins win the Stanley Cup Championship!"),
//            location: String::from("Pittsburgh, PA, USA"),
//            author: String::from("Iceburgh"),
//            content: String::from("The Pittsburgh Penguins once again are the best
//            hockey team in the NHL."),
//        }
//    } else {
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from("of course, as you probably already know, people"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}