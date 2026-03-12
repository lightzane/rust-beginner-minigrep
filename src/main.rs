use std::error::Error;
use std::{env, fs, process};

use minigrep::{search_case_insensitive, search_faster};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // * This works too! Since `.collect<T>()` is available
    // let args = env::args().collect::<Vec<String>>();

    // * `dbg!` macro will print the value of `args` to the console for debugging purposes
    // dbg!(args);

    // let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
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
        for line in search_faster(&query, &contents) {
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

    // * Uses "clone" (sacrifices runtime performance for simplicity) to create new String instances from the arguments
    // pub fn build<'a>(args: &'a [String]) -> Result<Config, &'a str> // lifetime elision
    // pub fn build(args: &[String]) -> Result<Config, &str> { // shorthand, due to lifetime elision, Rust assumes it's the same as the above line
    //     if args.len() < 3 {
    //         // panic!("⛔ not enough arguments");
    //         return Err("⛔ not enough arguments");
    //     }

    //     let query = args[1].clone(); // ⚠️ `.clone()` for now, let's sacrifice runtime performance for simplicity
    //     let file_path = args[2].clone();
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();

    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }

    pub fn build(
        mut args: impl Iterator<Item = String>, // The input is now an "owned" iterator (and not borrowed) or not a reference
                                                // Rust doesn't know as there is no input lifetime to attach the output `&str`
                                                // Hence, we must explicitly write `&'static str` instead of just `&str`
    ) -> Result<Config, &'static str> {
        // The `env::args()` returns an iterator
        args.next(); // skips the first argument, which is the program name
        // .next() returns an Option<Some(T), None>

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("⛔ Didn't get a query string"), // early return with error message
                                                                // Does not proceed to the next lines of code if there is an error
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("⛔ Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
