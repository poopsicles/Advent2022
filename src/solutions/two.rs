// Day 2: Rock Paper Scissors
// https://adventofcode.com/2022/day/2

use std::{fs::File, io::Read};

// Iterate through the file, splitting by whitespace,
// and check the player's turn from the opponent's play
//
// For Part 1, use:
// X - A, draw    X - B, lose    X - C, win
// Y - A, win     Y - B, draw    Y - C, lose
// Z - A, lose    Z - B, win     Z - C draw
//
// For Part 2, use:
// X - A, lose    X - B, lose    X - C, lose
// Y - A, draw    Y - B, draw    Y - C, draw
// Z - A, win     Z - B, win     Z - C, win
//
// Match statements are super-inelegant...but hey, they
// probably provide more information for the complier
// than fiddling with ASCII values and looking up win-values
// in an array (my original solution)
// 
// Edit: Nope, just checked the Rust subreddit and I feel ashamed.

// Output:
// Using the guessed decryption, you scored 40 points!
// 4 won, 2 lost, and 1 drew.
//
// Using the correct decryption, you scored 25 points!
// 1 won, 2 lost, and 4 drew.

pub fn solve(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    let mut opp = ' ';

    file.read_to_string(&mut contents).unwrap();

    let mut score_one: u32 = 0;
    let mut stats_one = (0u16, 0u16, 0u16); // won, lost, drew

    let mut score_two: u32 = 0;
    let mut stats_two = (0u16, 0u16, 0u16);

    for ch in contents
        .split_ascii_whitespace()
        .map(|x| x.chars().next().unwrap())
    {
        match (ch, opp) {
            (x @ 'A'..='C', _) => opp = x,

            ('X', 'A') => {
                score_one += 4;
                stats_one.2 += 1;

                score_two += 3;
                stats_two.1 += 1;
            }
            ('X', 'B') => {
                score_one += 1;
                stats_one.1 += 1;

                score_two += 1;
                stats_two.1 += 1;
            }
            ('X', 'C') => {
                score_one += 7;
                stats_one.0 += 1;

                score_two += 2;
                stats_two.1 += 1;
            }

            ('Y', 'A') => {
                score_one += 8;
                stats_one.0 += 1;

                score_two += 4;
                stats_two.2 += 1;
            }
            ('Y', 'B') => {
                score_one += 5;
                stats_one.2 += 1;

                score_two += 5;
                stats_two.2 += 1;
            }
            ('Y', 'C') => {
                score_one += 2;
                stats_one.1 += 1;

                score_two += 6;
                stats_two.2 += 1;
            }

            ('Z', 'A') => {
                score_one += 3;
                stats_one.1 += 1;

                score_two += 8;
                stats_two.0 += 1;
            }
            ('Z', 'B') => {
                score_one += 9;
                stats_one.0 += 1;

                score_two += 9;
                stats_two.0 += 1;
            }
            ('Z', 'C') => {
                score_one += 6;
                stats_one.2 += 1;

                score_two += 7;
                stats_two.0 += 1;
            }

            _ => (),
        }
    }

    println!(
        "Using the guessed decryption, you scored {} points!",
        score_one
    );
    println!(
        "{} won, {} lost, and {} drew.\n",
        stats_one.0, stats_one.1, stats_one.2
    );
    println!(
        "Using the correct decryption, you scored {} points!",
        score_two
    );
    println!(
        "{} won, {} lost, and {} drew.",
        stats_two.0, stats_two.1, stats_two.2
    );
}
