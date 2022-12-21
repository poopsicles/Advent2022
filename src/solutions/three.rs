// Day 3: Rucksack Reorganization
// https://adventofcode.com/2022/day/3

// Iterate though every line, making the first half of
// each a set, then checking if every element in the second
// half is in the set.
//
// For Part 2, make the set for every three lines, then find
// their intersection.

// Output:
// The priorites of the items that need to be rearranged is 8243 in total.
// The priorites of the badge items is 2631 in total.

use std::{collections::HashSet, fs::File, io::Read};

pub fn solve(filename: &str) {
    let mut file = match File::open(filename) {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Error: does `{}` exist?", filename);
            return;
        }
    };
    
    let mut contents = String::new();
    let mut letter_set = HashSet::with_capacity(52);
    let mut rearrange_priority = 0;

    file.read_to_string(&mut contents).unwrap();

    // Part 1
    for line in contents.lines() {
        let len = line.len();

        // frequency count for first half
        line[0..len / 2].chars().for_each(|x| {
            letter_set.insert(x);
        });

        // lookup for second half
        for letter in line[len / 2..].chars() {
            if letter_set.contains(&letter) {
                rearrange_priority += if letter.is_ascii_uppercase() {
                    letter as u16 - 38
                } else {
                    letter as u16 - 96
                };

                break;
            }
        }

        // clear
        letter_set.clear();
    }

    println!("The priorites of the items that need to be rearranged is {} in total.", rearrange_priority);

    // Part 2
    let mut one_set = HashSet::with_capacity(52);
    let mut two_set = HashSet::with_capacity(52);
    let mut badge_priority = 0;

    for line in contents.lines().collect::<Vec<_>>().chunks_exact(3) {
        line[0].chars().for_each(|x| {one_set.insert(x);});
        line[1].chars().for_each(|x| { two_set.insert(x);});
        
        let common = one_set.intersection(&two_set).collect::<Vec<_>>();

        for letter in line[2].chars() {
            if common.contains(&&letter) {
                badge_priority += if letter.is_ascii_uppercase() {
                    letter as u16 - 38
                } else {
                    letter as u16 - 96
                };

                break;
            }
        }

        one_set.clear();
        two_set.clear();
    }

    println!("The priorites of the badge items is {} in total.", badge_priority);
}
