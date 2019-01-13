use std::env;
use std::process;

use city_spellcheck;

fn main() {
    if let Err(e) = city_spellcheck::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
