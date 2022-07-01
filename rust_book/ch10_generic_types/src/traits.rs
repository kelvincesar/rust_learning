pub trait Summary {
    fn summarize_author(&self) -> String; 
    // Implementação padrão
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    /* // Removido para ser possível ver o defaullt value atuar da trait
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.headline)
    }
    */
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn notify (item: &impl Summary){
    println!("[notify] Breaking news! {}", item.summarize());
}

/*
// outra maneira:
pub fn notify<T: Summary>(item: &T){
    println!("[notify] Breaking news! {}", item.summarize());
}
*/

use std::fmt::{Debug, Display};

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    //....
    return 12 
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("teste_1"),
        content: String::from("Hello teste1 !!"),
        reply: false,
        retweet: false
    }
}
pub fn run () {
    println!("Running traits!\n");
    let tweet = Tweet {
        username: String::from("kelvinc"),
        content: String::from("Hello rust !!"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("Kelvin Andrade"),
        headline: String::from("Título interessante"),
        content: String::from("Conteúdo")
    };

    println!("Tweet: {}", tweet.summarize());
    println!("Article: {}", article.summarize());

    notify(&article);

    println!("A new summarize returned: {}", returns_summarizable().summarize())
}