use itertools::sorted;

// Calorie Counting
// https://adventofcode.com/2022/day/1

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
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

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 1);
    assert_eq!(part1(input), Some(70374));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 1);
    assert_eq!(part2(input), Some(204610));
}
