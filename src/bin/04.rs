use itertools::Itertools;

// Camp Cleanup
// https://adventofcode.com/2022/day/4

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<u32> {
    let num_full_overlaps = input
        .lines()
        .map(read_ranges)
        .map(|(a, b)| -> (Range, Range) {
            if a.length() <= b.length() {
                (a, b)
            } else {
                (b, a)
            }
        })
        .map(|(smallest, longest)| {
            (longest.start <= smallest.start && smallest.end <= longest.end) as u32
        })
        .sum::<u32>();

    Some(num_full_overlaps)
}

fn part2(input: &str) -> Option<u32> {
    let num_overlaps = input
        .lines()
        .map(read_ranges)
        .map(|(a, b)| -> (Range, Range) {
            if a.start <= b.start {
                (a, b)
            } else {
                (b, a)
            }
        })
        .map(|(leftmost, rightmost)| (rightmost.start <= leftmost.end) as u32)
        .sum::<u32>();

    Some(num_overlaps)
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Range { start, end }
    }

    fn length(&self) -> u32 {
        self.end - self.start
    }
}

fn read_ranges(line: &str) -> (Range, Range) {
    // 1-3,5-8 -> ((1, 3), (5, 8))
    line.split(',')
        .map(|rng| {
            rng.split('-')
                .map(|section| section.parse::<u32>().unwrap())
                .tuples()
                .map(|(start, end)| Range::new(start, end))
                .nth(0)
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 4);
    assert_eq!(part1(input), Some(526));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 4);
    assert_eq!(part2(input), Some(886));
}
