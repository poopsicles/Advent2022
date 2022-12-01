// Day 1: Calorie Counting
// https://adventofcode.com/2022/day/1

use std::{fs::File, io::Read};

pub fn one(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    let mut calories = Vec::from([0u32]);

    file.read_to_string(&mut contents).unwrap();
    let mut current_elf = false;

    for line in contents.lines() {
        if line == "" {
            current_elf = false;
            continue;
        }

        let line_parse = line.parse::<u32>().unwrap();

        if current_elf {
            let current_cal = calories.pop().unwrap();
            calories.push(current_cal + line_parse);
        } else {
            calories.push(line_parse);
            current_elf = true
        }
    }

    let mut table = calories.iter().enumerate().collect::<Vec<(_, _)>>();
    table.sort_unstable_by_key(|&(_, cal)| cal);
    table.reverse();

    let mut sum = 0;
    for item in table.iter().take(3) {
        println!("Elf #{} with {} calories.", item.0, item.1);
        sum += item.1;
    }

    println!("\nSum of top three calories: {}", sum);
}
