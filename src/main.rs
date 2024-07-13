
use std::env;
use std::process;

use ripgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|_err| {
        //println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In the file {}", config.file_path);

    if let Err(_e) = ripgrep::run(config) {
        //println!("Application Error: {e}");
        process::exit(1);
    }

}


