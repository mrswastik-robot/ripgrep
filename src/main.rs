
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    //Reading the file!
    let contents = fs::read_to_string(config.file_path)
        .expect("Wasn't able to read the contents of the file.");

    println!("\nWith text:\n{contents}");


    struct Config {
        query: String,
        file_path: String,
    }

    fn parse_config(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

