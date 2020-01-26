use std::{env, fs};
use std::error::Error;


/*Define the basic config struct
 * Fileds:
 *      query: The query condition
 *      filename: The target file*/
pub struct Config {
   pub  query: String,
   pub filename: String,
   pub case_sensitive: bool,
}

/*The methods for Config struct*/
impl Config {
    /*********************verion 0.2****************************
    // Return a Result and the Err message's liftime is static
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments must be greater than 3!!!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // Get from environment
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }
    ***************************************************************/
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();    // Pop the first argument

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't pass a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't pass a file"),
        };

        // `CASE` is true if there is no CASE_SENSITIVE env var
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        // println!("Case: {}", case_sensitive);
        Ok(Config {query, filename, case_sensitive})
    }
}

// Parse the arguments into Config
//fn parse_config(args: &[String])) -> Config {
//    Config::new(args)
//}

// Extract the main logic from the main function
// Use the Box<dyn Error> trait object to get a
// type impls `Error`
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/*The search action to get the query from target file*/
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*****************version 0.1************************
    // 函数定义中返回值与contens具有相同的生命周期，所以
    // 返回值包含的应该是 contents 的string slice
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
    ****************************************************/
    contents.lines()
        .filter(|line| line.contains(&query))
        .collect()
}

/*Search case insentive in the file*/
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*****************version 0.1******************
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
    ************************************************/
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive,
Pick here";
        assert_eq!(
            vec!["safe, fast, productive,"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insentive() {
        let query = "rUSt";
        let contents = "\


Rust,
safe, fast, productive,
Pick here,
Trust me";
        assert_eq!(
            vec!["Rust,", "Trust me"],
            search_case_insensitive(query, contents)
        );

    }
}
