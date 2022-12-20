use itertools::sorted;

pub fn main() {
    println!("Calories");

    let input = advent_of_code::read_file("inputs", 1);

    advent_of_code::solve!(1, part1, &input);
    advent_of_code::solve!(2, part2, &input);
}

fn part1(input: &str) -> Option<u32> {
    let elves = parse(input);
    let max_calories = elves.into_iter().max().unwrap();
    Some(max_calories)
}

fn part2(input: &str) -> Option<u32> {
    let elves = parse(input);
    let total_top3_calories = sorted(elves).rev().take(3).sum();
    Some(total_top3_calories)
}

fn parse(input: &str) -> Vec<u32> {
    let mut elves = Vec::new();
    let mut calories = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    elves
}
