use std::io::stdin;

mod config;
mod error;
mod names;
mod parser;
mod schedule;

fn main() {
    println!("Enter a cron string to parse:");

    let mut s = String::new();

    stdin()
        .read_line(&mut s)
        .expect("failed to read line from stdin");

    let s = s.trim();
    let schedule = parser::parse(s);

    match schedule {
        Ok(schedule) => {
            println!("Successfully parsed, schedule is:\n{}", schedule);
        }

        Err(err) => {
            println!("Parse error encountered:\n{}", err);
        }
    }
}
