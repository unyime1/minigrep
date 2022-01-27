use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    // Collect command line arguments from terminal.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
