use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let query = config.query;
    let file_path = config.file_path;

    println!("In file {:?}", query);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text = {:?}", &contents);
    dbg!(&args);
    println!("{query:?}");
}

struct Config {
    query: String,
    file_path: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {query, file_path}
}
