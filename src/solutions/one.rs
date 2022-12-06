// Day 1: Calorie Counting
// https://adventofcode.com/2022/day/1

// Iterate through the file, adding the numbers to
// the previous entry in a list and going to the next element
// when a blank line is encountered, then sort in descending order
// and get the first, and sum the first 3

// Output:
// Elf #65 with 70698 calories.
// Elf #199 with 69773 calories.
// Elf #18 with 66172 calories.

// Sum of top three elves: 206643

use std::{fs::File, io::Read};

pub fn solve(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    let mut calories = Vec::from([0u32]); // offset so we can use indices as elf IDs

    file.read_to_string(&mut contents).unwrap();
    let mut current_elf = false; // tracks whether to add to the current elf or make a new entry

    for line in contents.lines() {
        let line_parse = match line.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                current_elf = false;
                continue;
            }
        };

        if current_elf {
            *calories.last_mut().unwrap() += line_parse
        } else {
            calories.push(line_parse);
            current_elf = true
        }
    }

    let mut table = calories.iter().enumerate().collect::<Vec<(_, _)>>();
    table.sort_unstable_by_key(|&(_, cal)| cal);
    table.reverse();

    let mut sum = 0;
    for &(id, cal) in table.iter().take(3) {
        println!("Elf #{} with {} calories.", id, cal); // Part 1
        sum += cal;
    }

    println!("\nSum of top three calories: {}", sum); // Part 2
}
