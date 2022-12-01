mod solutions;

use std::io::{self, Write};

fn main() {
    println!("<All>\t<#1>\n");
    print!("What day? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.to_lowercase().trim() {
        "all" => {
            println!("Day #1");
            solutions::one::solve("inputs\\one.txt");
            println!();
        }

        "1" => solutions::one::solve("inputs\\one.txt"),

        _ => println!("Invalid, oops!"),
    }
}
