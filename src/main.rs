mod solutions;

use std::io::{self, Write};

fn main() {
    println!("<All>\t<#1>\t<#2>\n");
    print!("What day? (all/q/#): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.to_lowercase().trim() {
        "q" => (),

        "all" => {
            println!("\n----- Day #1 -----");
            solutions::one::solve("inputs\\one.txt");
            
            println!("\n----- Day #2 -----");
            solutions::two::solve("inputs\\two.txt");
        }

        "1" => solutions::one::solve("inputs\\one.txt"),
        "2" => solutions::two::solve("inputs\\two.txt"),

        _ => println!("Invalid, oops!"),
    }
}
