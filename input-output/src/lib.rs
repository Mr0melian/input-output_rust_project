use std::{error::Error, fs, env};

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive: bool,
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("no more arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn run (config: Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in result{
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


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
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
            filename: "c".to_string(),
            case_sensitive: true
        };
        assert_eq!(con.query, config.query);
    }
    #[test]
    fn test_run(){
        let config = Config{
            query: "b".to_string(),
            filename: "example.txt ".to_string(),
            case_sensitive: true
        };
        if let Err(e) = run(config) {
            assert!(true)
        }
    }

   

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        
Rust:
safe, fast, productive.
Pick three.
Duct type.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}