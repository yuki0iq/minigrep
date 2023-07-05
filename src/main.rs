use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing command line arguments: {err}");
        process::exit(1);
    });

    println!("find {} in file {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text {}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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
