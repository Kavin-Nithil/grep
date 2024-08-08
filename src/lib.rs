use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments.");
        } else if args.len() > 3 {
            panic!("Extra arguments provided");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query,
            file_path
        }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        } else if args.len() > 3 {
            return Err("Extra arguments provided");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn is short for dynamic
    let contents = fs::read_to_string(config.file_path)?;

    println!("Contents: \n {:?}", contents);

    Ok(())
}
