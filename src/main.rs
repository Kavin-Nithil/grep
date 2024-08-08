use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = mini_grep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = mini_grep::run(config) {
        eprintln!(" Encountered the following error {:?}", e);
        process::exit(1);
    };
}
