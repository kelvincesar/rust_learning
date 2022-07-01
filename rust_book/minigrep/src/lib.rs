use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(&config.file)?;
    println!("Resultados:");
    
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static  str> {
        // ignore program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Não foi possível localizar a `query`")
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Não foi possível localizar o `file_name`")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, file, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }
    results 
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents));
    }
}
