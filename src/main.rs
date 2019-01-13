#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate simple_logger;

use std::process;

mod args;
mod commands;
mod error;

fn main() {
    match args::parse() {
        Ok(s) => println!("{}", s),
        Err(e) => { println!("error: {}", e); process::exit(1); },
    }
}
