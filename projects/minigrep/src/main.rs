use std::{env, fs, process};
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    // closure example:
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // eprintln! macro used to output error messages to terminal instead of possibly some file
        process::exit(1);
    });

    // only handle the error case here because run doesn't produce anything in the Ok case
    if let Err(e) = run(config) {
        eprintln!("Application error {e}"); // eprinlin! used here for same reason as above
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(()) // not producing any output because we're using run for its side effects only
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}
