use std::collections::HashSet;

// Tuning Trouble
// https://adventofcode.com/2022/day/6

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<usize> {
    Some(find_marker_start(4, input))
}

fn part2(input: &str) -> Option<usize> {
    Some(find_marker_start(14, input))
}

fn find_marker_start(size: usize, input: &str) -> usize {
    let mut chars = input.chars();
    let mut window = Vec::new();
    let mut num_processed = 0;

    loop {
        let c = chars.next().unwrap();
        num_processed += 1;
        window.push(c);

        if num_processed <= size {
            continue;
        }

        window.remove(0);
        assert!(window.len() == size);

        let unique_chars: HashSet<char> = window.clone().into_iter().collect();

        if window.len() == unique_chars.len() {
            return num_processed;
        }
    }
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 6);
    assert_eq!(part1(input), Some(1640));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 6);
    assert_eq!(part2(input), Some(3613));
}
