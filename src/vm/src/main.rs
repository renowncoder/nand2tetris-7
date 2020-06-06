extern crate vm;

use std::env;

use vm::{config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
