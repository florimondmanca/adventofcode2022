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

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new("inputs/3.txt"))?;

    let mut sum_of_priorities = 0;

    for line in content.lines() {
        assert!(line.len() % 2 == 0, "expected line to have even length");
        let compartment_size = line.len() / 2;

        let (left, right) = line.split_at(compartment_size);

        let left_chars: HashSet<char> = HashSet::from_iter(left.chars());
        let right_chars: HashSet<char> = HashSet::from_iter(right.chars());

        let common_item_types: Vec<&char> = left_chars.intersection(&right_chars).collect();

        assert!(
            common_item_types.len() == 1,
            "expected only one common item type"
        );

        sum_of_priorities += get_priority(common_item_types[0].clone());
    }

    println!("Answer (part 1): {}", sum_of_priorities);

    Ok(())
}
