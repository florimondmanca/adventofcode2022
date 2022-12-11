use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let content = include_str!("inputs/03.txt");

    let common_sum = content
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

    println!("Answer (part 1): {common_sum}");

    let badges_sum = content
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

    println!("Answer (part 2): {badges_sum}");
}

fn get_priority(item_type: char) -> u32 {
    let code = item_type as u32;

    if item_type.is_lowercase() {
        return code - 'a' as u32 + 1;
    } else {
        return code - 'A' as u32 + 27;
    }
}
