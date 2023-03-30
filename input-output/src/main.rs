use std::env;
use std::fs;
struct Config{
    query:String,
    filename:String,
}

impl Config{
    fn new(args: &[String]) -> Config{
        if args.len() < 3{
            panic!("no arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args);
    println!("Serching query: {}", config.query);
    println!("In file: {}", config.filename);
    let contest = fs::read_to_string(config.filename).expect("Failed to read");
    println!("With text:\n{}", contest);
}
