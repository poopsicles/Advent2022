# [Advent of Code 2022](https://adventofcode.com/2022/) (but like, ✨pretty✨)

My AoC solutions...in Rust

Currently ~~WIP~~ struggling with coming back after day 7.

My personal aim for this is to get used to writing and reading (performant) Rust, and using things in the standard library...like say, [VecDeques](https://doc.rust-lang.org/std/collections/struct.VecDeque.html), for instance.

But yeah, standard library only. So no [itertools](https://docs.rs/itertools/latest/itertools/) (definitely would have been helpful) or [bitflags](https://docs.rs/bitflags/latest/bitflags/index.html) (we'll just have to manage with arrays) or anything. I realise that IRL projects obviously aren't written like this but hey, lemme at least use `Iterator::collect::<Vec<_>>().chunks_exact(4)` a couple times manually before running to a crate 😂

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

Actually, no, I just want an excuse to not go for the leaderboards haha 😪.

## Running the code

I made a little interface to make running it easier ([install the Rust toolchain](https://rustup.rs) if you haven't).

You'll also need to get your own inputs from the [AoC website](https://adventofcode.com/2022) and save them in a folder called `inputs` at the project root after cloning (like `day_number.txt`), due to the terms:

> The design elements, language, styles, and concept of Advent of Code are all the sole property of Advent of Code and may not be replicated or used by any other person or entity without express written consent of Advent of Code.

```text
$ git clone https://github.com/poopsicles/Advent2022.git && cd Advent2022

$ mkdir inputs && cp ~/Downloads/5.txt inputs

$ cargo run --release
<All>

<#1>    <#2>    <#3>    <#4>    <#5>

What day? (all/q/#): 5
The visualisation for CraneMover 9000 is:

[J]                             [J] 
[V]                             [P] 
[R]                             [D] 
[R]                             [D] 
[L]                             [B] 
[H]                             [C] 
[S]                             [L] 
[P]                             [Z] 
[D]                             [J] 
[B]                             [V] 
[V] [D]                         [B] 
[V] [Q] [T]                     [R] 
[G] [H] [Z]                     [F] 
[Z] [P] [N]     [R]             [W] 
[V] [Q] [N]     [P]             [C] 
[R] [L] [V]     [G]     [C]     [J] 
[D] [H] [F] [M] [M] [W] [L] [Q] [W] 
 1   2   3   4   5   6   7   8   9   

The visualisation for CraneMover 9001 is:

[V]                             [D]
[Q]                             [P]
[R]                             [J]
[Z]                             [S]
[Z]                             [W]
[R]                             [J]
[P]                             [L]
[V]                             [G]
[V]                             [C]
[M]                             [M]
[D] [H]                         [W]
[G] [H] [J]                     [L]
[Q] [V] [F]                     [V]
[Z] [P] [F]     [D]             [B]
[L] [N] [C]     [R]             [Q]
[R] [B] [D]     [B]     [W]     [H]
[L] [V] [N] [D] [J] [C] [P] [R] [T]
 1   2   3   4   5   6   7   8   9

...Completed in 14ms
```
