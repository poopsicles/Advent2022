# [Advent of Code 2022](https://adventofcode.com/2022/) (but like, ✨pretty✨)

My AoC solutions...in Rust

Currently WIP.

I love making the results look nice so form over functionality alwaysssssssss (a.k.a. inefficient nice-looking outputs will always be preferred the absolute-fastest-but-like-just-the-answer-is-shown thing).

But obviously, gotta go fast, right?

Like take [Day 1](src/solutions/one.rs) (simplified) for example:

```rust
let calories = Vec::new(); // for the elves and how many calories they have
// ~snip~
let mut table = calories.iter().enumerate().collect(); // to get the elf #'s
table.sort();
table.reverse();

let mut sum = 0;
for &(id, cal) in table.iter().take(3) {
    println!("Elf #{} with {} calories.", id, cal);
    sum += cal;
}
```

While getting the elf IDs themselves wasn't necessary (just the sum was, a simple `take(3).sum()` would have sufficed), I did it because nice user-facing output makes a good application.

```text
Elf #65 with 70698 calories.
Elf #199 with 69773 calories.
Elf #18 with 66172 calories.
```

Actually, no, I just want an excuse to not go for the leaderboards haha 😂.

## Running the code

I made a little interface to make running it easier ([install the Rust toolchain](https://rustup.rs) if you haven't)

```text
$ git clone https://github.com/poopsicles/Advent2022.git && cd Advent2022

$ cargo run
<All>   <#1>    <#2>

What day? (all/q/#): all

----- Day #1 -----
Elf #65 with 70698 calories.
Elf #199 with 69773 calories.
Elf #18 with 66172 calories.

Sum of top three calories: 206643

----- Day #2 -----
(continued)
```
