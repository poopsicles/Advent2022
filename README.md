# Advent of Code 2022

My AoC solutions...

Currently WIP.

I love making the results look nice so form over functionality alwaysssssssss (a.k.a. inefficient nice-looking outputs will always be preferred the absolute-fastest-but-no-output thing)

Like take [Day 1](src/solutions/one.rs) (simplified) for example:

```rust
let calories = Vec::new(); // for the elves and how many calories they have
// ~snip~
let mut table = calories.iter().enumerate().collect(); // to get the elf #'s
table.sort();
table.reverse();

let mut sum = 0;
for item in table.iter().take(3) {
    println!("Elf #{} with {} calories.", item.0, item.1);
    sum += item.1;
}

println!("\nSum of top three calories: {}", sum);
```

While getting the elf IDs themselves wasn't necessary (just the sum was, a simple `take(3).sum()` would have sufficed), I did it because nice user-facing output makes a good application.

Actually, no, I just want an excuse to use `println!()` a lot 😂.
