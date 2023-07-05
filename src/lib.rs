use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // TODO make more 'readable' error message
            Err("Not enough arguments")
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text {}", contents);

    Ok(())
}

