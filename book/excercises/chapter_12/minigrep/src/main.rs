use std::error::Error;
use std::{env, fs, process};

use minigrep::{search, search_insensitive};

fn main() {
    // Test retrieving arguments passed into the command line with env. First argument usual is the
    // path used to call the binary
    // let args = env::args();
    // for arg in args {
    // println!("{arg}");
    // }

    // let args: Vec<String> = env::args().collect();
    //dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];
    //
    // std::env::args() returns an iterator over Strings
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{}", contents);

    let results = if config.ignore_casing {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_casing: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a path string."),
        };

        let ignore_casing = env::var("IGNORE_CASING").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_casing,
        })
    }
}
