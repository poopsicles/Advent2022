// Day 5: Supply Stacks
// https://adventofcode.com/2022/day/5

// Parse the stacks into lists, then push and
// pop as needed (put crates in a buffer for Part 2)

// Output:
// The visualisation for CraneMover 9000 is:
//
// [J]                             [J]
// [V]                             [P]
// [R]                             [D]
// [R]                             [D]
// [L]                             [B]
// [H]                             [C]
// [S]                             [L]
// [P]                             [Z]
// [D]                             [J]
// [B]                             [V]
// [V] [D]                         [B]
// [V] [Q] [T]                     [R]
// [G] [H] [Z]                     [F]
// [Z] [P] [N]     [R]             [W]
// [V] [Q] [N]     [P]             [C]
// [R] [L] [V]     [G]     [C]     [J]
// [D] [H] [F] [M] [M] [W] [L] [Q] [W]
//  1   2   3   4   5   6   7   8   9
//
// The visualisation for CraneMover 9001 is:
//
// [V]                             [D]
// [Q]                             [P]
// [R]                             [J]
// [Z]                             [S]
// [Z]                             [W]
// [R]                             [J]
// [P]                             [L]
// [V]                             [G]
// [V]                             [C]
// [M]                             [M]
// [D] [H]                         [W]
// [G] [H] [J]                     [L]
// [Q] [V] [F]                     [V]
// [Z] [P] [F]     [D]             [B]
// [L] [N] [C]     [R]             [Q]
// [R] [B] [D]     [B]     [W]     [H]
// [L] [V] [N] [D] [J] [C] [P] [R] [T]
//  1   2   3   4   5   6   7   8   9

use std::{collections::VecDeque, fs::File, io::Read};

pub fn solve(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // get number of stacks
    let stack_num = (contents.bytes().take_while(|&x| x != 10).count() + 1) / 4;
    let mut stacks_one: Vec<VecDeque<char>> = Vec::with_capacity(stack_num);

    for _ in 0..stack_num {
        stacks_one.push(VecDeque::new())
    }

    // Parse crates
    contents
        .lines()
        .take_while(|x| !x.starts_with(" 1"))
        .for_each(|line| {
            let crates = line.chars().skip(1).step_by(4).collect::<Vec<_>>();
            for i in 0..stack_num {
                if crates[i] != ' ' {
                    stacks_one[i].push_front(crates[i]);
                }
            }
        });

    let mut stacks_two = stacks_one.clone();

    // Parse instructions
    contents
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .map(|line| {
            line.split(" ")
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|set| {
            // Part 1
            for _ in 0..set[0] {
                let moved = stacks_one[set[1] - 1].pop_back().unwrap();
                stacks_one[set[2] - 1].push_back(moved);
            }

            // Part 2
            let mut buffer = Vec::with_capacity(set[0]);
            for _ in 0..set[0] {
                buffer.push(stacks_two[set[1] - 1].pop_back().unwrap())
            }
            for _ in 0..buffer.len() {
                stacks_two[set[2] - 1].push_back(buffer.pop().unwrap());
            }
        });

    // Print Part 1
    let max_height_one = stacks_one.iter().map(|x| x.len()).max().unwrap();
    let mut visual_one = String::from(" ");

    for i in 0..stack_num {
        visual_one = format!("{}{}   ", visual_one, i + 1)
    }

    for _ in 0..max_height_one {
        visual_one = format!("\n{}", visual_one);

        for j in (0..stack_num).rev() {
            match stacks_one[j].pop_front() {
                Some(x) => visual_one = format!("[{}] {}", x, visual_one),
                None => visual_one = format!("    {}", visual_one),
            }
        }
    }

    // Print Part 2
    let max_height_two = stacks_two.iter().map(|x| x.len()).max().unwrap();
    let mut visual_two = String::from(" ");

    for i in 0..stack_num {
        visual_two = format!("{}{}   ", visual_two, i + 1)
    }

    for _ in 0..max_height_two {
        visual_two = format!("\n{}", visual_two);

        for j in (0..stack_num).rev() {
            match stacks_two[j].pop_front() {
                Some(x) => visual_two = format!("[{}] {}", x, visual_two),
                None => visual_two = format!("    {}", visual_two),
            }
        }
    }

    println!("The visualisation for CraneMover 9000 is:\n");
    println!("{}", visual_one);
    println!();
    println!("The visualisation for CraneMover 9001 is:\n");
    println!("{}", visual_two);
}
