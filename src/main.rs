use snif::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with parsing arguments: {}", err);
        process::exit(1);
    });

    println!();
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!();

    if let Err(e) = snif::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
