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
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents){
        println!("{}", line);
    }
    Ok(())
}
 
pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let s = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let config = Config::new(&s).unwrap();
        let con = Config{
            query: "b".to_string(),
            filename: "c".to_string()
        };
        assert_eq!(con.query, config.query);
    }
    #[test]
    fn test_run(){
        let config = Config{
            query: "b".to_string(),
            filename: "example.txt ".to_string()
        };
        if let Err(e) = run(config) {
            assert!(true)
        }
    }

   

    #[test]
    fn one_result() {
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
}