use std::io::{stdin};

mod parser;
mod schedule;
mod names;
mod error;

fn main() {
    println!("Enter a cron string to parse:\n");

    let mut s = String::new();

    stdin().read_line(&mut s)
        .expect("failed to read line from stdin");

    println!("Parsing [{}] ...", s);

    // do some parsing
}
