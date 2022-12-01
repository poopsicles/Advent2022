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

    calories.sort();
    calories.reverse();

    println!("{}", calories.iter().take(3).sum::<u32>())
}
