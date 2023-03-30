use std::env;
use std::process;

use input_output::Config;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("truble in deconstruct arguments: {}", err);
        process::exit(1);
    });
    println!("serching query:{}", config.query);
    println!("in file:{}", config.filename);
    if let Err(e) = input_output::run(config) {
        println!("Error in app: {}", e);
        process::exit(1);
    }
}





