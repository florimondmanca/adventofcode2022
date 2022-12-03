use itertools::sorted;
use std::fs;
use std::path::Path;

pub fn run() {
    let content = fs::read_to_string(Path::new("inputs/1.txt")).unwrap();

    let mut elves = Vec::new();

    {
        let mut calories = 0;

        for line in content.lines() {
            if line.is_empty() {
                elves.push(calories);
                calories = 0;
            } else {
                calories += line.parse::<i32>().unwrap();
            }
        }
    }

    let max_calories = elves.iter().max().unwrap();
    println!("Answer (part 1): {}", max_calories);

    let total_top3_calories = sorted(elves).rev().take(3).sum::<i32>();
    println!("Answer (part 2): {}", total_top3_calories);
}
