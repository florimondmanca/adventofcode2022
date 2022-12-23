use itertools::sorted;

pub fn main() {
    println!("Calories");
    let input = advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part1, &input);
    advent_of_code::solve!(2, part2, &input);
}

fn part1(input: &str) -> Option<u32> {
    let calories = parse(input);
    let max_calories = calories.into_iter().max().unwrap();
    Some(max_calories)
}

fn part2(input: &str) -> Option<u32> {
    let calories = parse(input);
    let total_top3_calories = sorted(calories).rev().take(3).sum();
    Some(total_top3_calories)
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|text| text.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect()
}
