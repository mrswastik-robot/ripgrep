
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
    
    impl Config {
        
        fn build (args: &Vec<String>) -> Result<Config, &str> {
            
            if args.len() < 3
            {
                return Err("not enough parameters");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
        }

    }
   
}

