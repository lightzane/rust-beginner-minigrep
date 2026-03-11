use std::error::Error;
use std::{env, fs, process};

use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    // * This works too! Since `.collect<T>()` is available
    // let args = env::args().collect::<Vec<String>>();

    // * `dbg!` macro will print the value of `args` to the console for debugging purposes
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        // When running `cargo run > output.txt`
        // `println!` will write the error message into output.txt, which is not what we want
        // println!("⛔ Problem parsing arguments: {err}");

        // 'eprintln!' --> will print error into standard error
        // and will not be written into a output.txt
        eprintln!("⛔ Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("⛔ Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let query = config.query;
    let file_path = config.file_path;
    let ignore_case = config.ignore_case;

    println!();
    println!("Searching for \x1b[1;36m{query}\x1b[0m");
    println!("In file \x1b[1;36m{file_path}\x1b[0m");

    // * Custom error message that this function will handle
    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    // * Returning the error to the caller instead of handling it here
    let contents = fs::read_to_string(file_path)?;
    /*
        `?` operator will immediately return the error "E" from Result<T, E>
        to the caller if there is an error,
        otherwise it will unwrap the value "T" and assign it to `contents`

        `?` is usually used in functions that return a `Result` type, and it will propagate the error
    */

    println!("\n------ START ------\n");
    println!("\x1b[1m{contents}\x1b[0m");
    println!("\n------ END ------\n");

    if ignore_case {
        for line in search_case_insensitive(&query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&query, &contents) {
            println!("{line}");
        }
    }

    Ok(contents)
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // Use `build` instead of `new` since many programmers expect `new` to never fail
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("⛔ not enough arguments");
            return Err("⛔ not enough arguments");
        }

        let query = args[1].clone(); // ⚠️ `.clone()` for now, let's sacrifice runtime performance for simplicity
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
