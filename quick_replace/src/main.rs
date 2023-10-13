use std::fs;

use text_colorizer::Colorize;
use crate::parse_args::parse_args;
use crate::replace::replace;

mod arguments;
mod print_usage;
mod parse_args;
mod replace;

fn main() {
    let args = parse_args();
    let data = match fs::read_to_string(&args.filename) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), &args.filename, e);
            std::process::exit(1);
        }
    };
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), &args.filename, e);
            std::process::exit(1);
        }
    }
}
