// Day 4: Camp Cleanup
// https://adventofcode.com/2022/day/4

// Deserialise the file into a vector, then take four numbers at a time
// [a, b, c, d] where a..=b is one range, and c..=d is another
// 
// For Part 1:
// if c and d are less than/equal to a and b respectively, 
// it is contained in it and vice-versa
//
// For Part 2:
// if c in a..b (c is greater than/equal to a and less than/equal to b)
// and vice-versa, there's an overlap

use std::{fs::File, io::Read};

pub fn solve(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let mut total_overlap = 0u16;
    let mut partial_overlap = 0u16;

    contents
        .split(&['-', ',', '\n'])
        .map(|x| x.parse::<u8>().unwrap_or_default())
        .collect::<Vec<_>>()
        .chunks_exact(4)
        .for_each(|x| {
            if x[0] <= x[2] && x[1] >= x[3] || x[2] <= x[0] && x[3] >= x[1] {
                total_overlap += 1;
            }

            if x[2] >= x[0] && x[2] <= x[1] || x[0] >= x[2] && x[0] <= x[3] {
                partial_overlap += 1;
            }
        });
        
    println!("There are {} assignment pairs in which one fully contains the other.", total_overlap);
    println!("There are {} assignment pairs in which the assignments overlap", partial_overlap);
}