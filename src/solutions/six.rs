// Day 6: Tuning Trouble
// https://adventofcode.com/2022/day/6

// Start from the beginning of the string and make a sub-string
// backwards (length depending on how many unique characters needed)
// and check if there's any duplicate letters

// Output:
// The start-of-packet marker is at character 1876.
// The start-of-message marker is at character 2202.

use std::{fs::File, io::Read};

pub fn solve(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    let mut packet = 0;
    let mut message = 0;

    file.read_to_string(&mut contents).unwrap();

    // Part 1
    for i in 4..contents.len() {
        let prev = &contents[i - 4..i];
        let mut letters = Vec::from(prev);
        letters.sort();
        letters.dedup();

        if letters.len() == 4 {
            packet = i;
            break;
        }
    }

    for i in 14..contents.len() {
        let prev = &contents[i - 14..i];
        let mut letters = Vec::from(prev);
        letters.sort();
        letters.dedup();

        if letters.len() == 14 {
            message = i;
            break;
        }
    }

    println!("The start-of-packet marker is at character {}.", packet);
    println!("The start-of-message marker is at character {}.", message);
}
