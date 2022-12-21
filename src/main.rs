mod solutions;

use std::io::{self, Write};
use std::time::Instant;

fn main() {
    if !std::path::Path::new("inputs").exists() {
        eprintln!("Error: you need a folder called `inputs` at the project root.")
    }

    println!("<All>\n\n<#1>\t<#2>\t<#3>\t<#4>\t<#5>");
    println!("<#6>\t<#7>\n");
    print!("What day? (all/q/#): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.to_lowercase().trim() {
        "q" => (),

        "all" => {
            let start = Instant::now();

            println!("\n----- Day #1 -----");
            solutions::one::solve("inputs\\1.txt");

            println!("\n----- Day #2 -----");
            solutions::two::solve("inputs\\2.txt");

            println!("\n----- Day #3 -----");
            solutions::three::solve("inputs\\3.txt");

            println!("\n----- Day #4 -----");
            solutions::four::solve("inputs\\4.txt");

            println!("\n----- Day #5 -----");
            solutions::five::solve("inputs\\5.txt");
            
            println!("\n----- Day #6 -----");
            solutions::six::solve("inputs\\6.txt");

            println!("\n----- Day #7 -----");
            solutions::seven::solve("inputs\\7.txt");

            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }

        "1" => {
            let start = Instant::now();
            solutions::one::solve("inputs\\1.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }
        "2" => {
            let start = Instant::now();
            solutions::two::solve("inputs\\2.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }
        "3" => {
            let start = Instant::now();
            solutions::three::solve("inputs\\3.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }
        "4" => {
            let start = Instant::now();
            solutions::four::solve("inputs\\4.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }
        "5" => {
            let start = Instant::now();
            solutions::five::solve("inputs\\5.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }
        "6" => {
            let start = Instant::now();
            solutions::six::solve("inputs\\6.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }
        "7" => {
            let start = Instant::now();
            solutions::seven::solve("inputs\\7.txt");
            let done = start.elapsed().as_millis();
            println!("\n...Completed in {}ms", done);
        }

        _ => println!("Invalid, oops!"),
    }
}
