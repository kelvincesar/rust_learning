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
}