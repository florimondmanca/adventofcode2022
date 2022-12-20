use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!("Rucksack Reorganization");

    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<u32> {
    let common_sum = input
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            [a, b].into_iter()
        })
        .map(|x| -> HashSet<char> { HashSet::from_iter(x.chars()) })
        .tuples()
        .map(|(left, right)| left.intersection(&right).cloned().nth(0).unwrap())
        .map(get_priority)
        .sum::<u32>();

    Some(common_sum)
}

fn part2(input: &str) -> Option<u32> {
    let badges_sum = input
        .lines()
        .map(|line| -> HashSet<char> { HashSet::from_iter(line.chars()) })
        .tuples()
        .map(|(elf0, elf1, elf2)| -> char {
            elf2.intersection(&elf0.intersection(&elf1).cloned().collect())
                .cloned()
                .nth(0)
                .unwrap()
        })
        .map(get_priority)
        .sum::<u32>();

    Some(badges_sum)
}

fn get_priority(item_type: char) -> u32 {
    let code = item_type as u32;

    if item_type.is_lowercase() {
        return code - 'a' as u32 + 1;
    } else {
        return code - 'A' as u32 + 27;
    }
}
