use std::{error::Error, fs};

pub struct Config{
    pub query:String,
    pub filename:String,
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("no more arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename})
    }
}

pub fn run (config: Config)->Result<(),Box<dyn Error>>{
    let contest = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contest);
    Ok(())
}