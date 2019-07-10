use std::env;
use std::fs;
use std::process;

struct Config
{
    query: String,
    filename: String,
}

// fn parse_config(args: &[String]) -> Config
// {
//     let query = args[1].clone();
//     let filename = args[2].clone();
// 
//     Config {query, filename}
// }

// 构造函数
impl Config 
{
    fn new(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3    // 检查错误 
        {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

       Ok(Config { query, filename })
    }
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    
    // let query = &args[1];
    // let filename = &args[2];
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err|
    {
        println!("Problem parsing arguments: {}", err);    // 这种检查错误的方式还是很精妙的
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}",contents);


}
