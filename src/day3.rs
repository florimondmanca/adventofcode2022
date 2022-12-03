use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

const ASCII_UPPERCASE_A: u32 = 65;
const ASCII_LOWERCASE_A: u32 = 97;

fn get_priority(item_type: char) -> u32 {
    let code = item_type as u32;

    if item_type.is_lowercase() {
        return code - ASCII_LOWERCASE_A + 1;
    } else {
        return code - ASCII_UPPERCASE_A + 27;
    }
}

fn part1(content: String) -> Result<(), Box<dyn Error>> {
    let mut sum_of_priorities = 0;

    for line in content.lines() {
        assert!(line.len() % 2 == 0, "expected line to have even length");
        let compartment_size = line.len() / 2;

        let (left, right) = line.split_at(compartment_size);

        let left_chars: HashSet<char> = HashSet::from_iter(left.chars());
        let right_chars: HashSet<char> = HashSet::from_iter(right.chars());

        let common_item_types: Vec<char> = left_chars.intersection(&right_chars).cloned().collect();

        assert!(
            common_item_types.len() == 1,
            "expected only one common item type"
        );

        sum_of_priorities += get_priority(common_item_types[0].clone());
    }

    println!("Answer (part 1): {}", sum_of_priorities);

    Ok(())
}

fn part2(content: String) -> Result<(), Box<dyn Error>> {
    let mut sum_of_priorities = 0;

    for chunk in &content.lines().chunks(3) {
        let lines: Vec<&str> = chunk.collect();
        let elf0: HashSet<char> = HashSet::from_iter(lines[0].chars());
        let elf1: HashSet<char> = HashSet::from_iter(lines[1].chars());
        let elf2: HashSet<char> = HashSet::from_iter(lines[2].chars());

        let mut common_item_types: HashSet<char> = elf0.intersection(&elf1).cloned().collect();
        common_item_types = elf2.intersection(&common_item_types).cloned().collect();

        assert!(
            common_item_types.len() == 1,
            "expected only one common item for all 3 elves"
        );

        let badge = common_item_types.iter().nth(0).unwrap();

        sum_of_priorities += get_priority(badge.clone());
    }

    println!("Answer (part 2): {}", sum_of_priorities);

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new("inputs/3.txt"))?;
    part1(content.clone())?;
    part2(content.clone())?;
    Ok(())
}
