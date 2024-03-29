use std::error::Error;
use std::fs;
use std::env;

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn one_result() 
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() 
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

     #[test]
    fn case_insensitive() 
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> 
{
    // let mut results = Vec::new();

    // for line in contents.lines() 
    // {
    //     if line.contains(query) 
    //     {
    //         results.push(line);
    //     }
    // }

    // results
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)     // 比较的每行都转换成小写
        {
            results.push(line);
        }
    }
    results
}

pub struct Config
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// fn parse_config(args: &[String]) -> Config
// {
//     let query = args[1].clone();
//     let filename = args[2].clone();
// 
//     Config {query, filename}
// }

// 构造函数
impl Config    // Config类
{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>
    {
        if args.len() < 3    // 检查错误 
        {
            return Err("not enough arguments");
        }

        // let query = args[1].clone();
        // let filename = args[2].clone();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

       Ok(Config { query, filename , case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> 
{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, & contents)
    };

    for line in search(&config.query, &contents)
    {
        println!("{}", line);
    }

    // println!("With text:\n{}", contents);

    Ok(())
}
