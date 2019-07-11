use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    
    // let query = &args[1];
    // let filename = &args[2];
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);    // 这种检查错误的方式还是很精妙的
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config){    // minigrep是工程名称,run是库lib.rs中的pub fn
        println!("Application error: {}", e);
        
        process::exit(1);
    }

    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // println!("With text:\n{}",contents);


}
