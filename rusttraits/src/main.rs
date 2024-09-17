// https://medium.com/@rs4528090/understanding-rusts-trait-system-2355941a5e31
use std::fmt::Display;

trait Summary {
    fn summarize(&self) ->String{
        String::from("Read more..")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize()); 
}


impl Summary for NewsArticle {
    fn summarize(&self) ->String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)

    }
    
}

// Multiple Trait bounds
fn display_info<T:Summary + Display>(item:&T){
    println!("Item information: {}", item);
}
fn main() {
    println!("Hello, world!");
}
