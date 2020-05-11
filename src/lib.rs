use std::{fs , error::Error};

pub struct Config{
    pub query: String,
    pub filename: String
}

impl Config{
    pub fn new(parameters: &[String]) -> Result<Config , &'static str>{
        if parameters.len() < 3{
            return Err("not enough arguments provided")
        }
        let query = parameters[1].clone();
        let filename = parameters[2].clone();

        Ok(Config{query , filename})
    }
}

pub fn run(config: Config) -> Result<() , Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    for line in search(&config.query , &content){
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str , contents: &'a str) -> Vec<&'a str>{
    let mut return_value = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            return_value.push(line);
        }
    }
    return_value
}



#[cfg(test)]
mod tests {
    use super::*;

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
