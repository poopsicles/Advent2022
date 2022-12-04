mod solutions;

use std::io::{self, Write};
use std::time::Instant;

fn main() {
    println!("<All>\t<#1>\t<#2>\t<#3>\n");
    print!("What day? (all/q/#): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.to_lowercase().trim() {
        "q" => (),

        "all" => {
            let start = Instant::now();

            println!("\n----- Day #1 -----");
            solutions::one::solve("inputs\\one.txt");

            println!("\n----- Day #2 -----");
            solutions::two::solve("inputs\\two.txt");

            println!("\n----- Day #3 -----");
            solutions::three::solve("inputs\\three.txt");

            let done = start.elapsed().as_micros();
            println!("\n...Completed in {}μs", done);
        }

        "1" => {
            let start = Instant::now();
            solutions::one::solve("inputs\\one.txt");
            let done = start.elapsed().as_micros();
            println!("\n...Completed in {}μs", done);
        }
        "2" => {
            let start = Instant::now();
            solutions::two::solve("inputs\\two.txt");
            let done = start.elapsed().as_micros();
            println!("\n...Completed in {}μs", done);
        }
        "3" => {
            let start = Instant::now();
            solutions::three::solve("inputs\\three.txt");
            let done = start.elapsed().as_micros();
            println!("\n...Completed in {}μs", done);
        }

        _ => println!("Invalid, oops!"),
    }
}
