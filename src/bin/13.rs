use itertools::Itertools;
use std::cmp::Ordering;

// Distress Signal
// https://adventofcode.com/2022/day/13

// Kudos: https://www.reddit.com/r/adventofcode/comments/zkmyh4/comment/j01mqo7/

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<usize> {
    let packets = parse(input);

    let sum = packets
        .into_iter()
        .tuples()
        .enumerate()
        .filter_map(|(i, (p1, p2))| (p1 <= p2).then(|| i + 1))
        .sum();

    Some(sum)
}

fn part2(input: &str) -> Option<usize> {
    let mut packets = parse(input);

    // Insert the dividers, then sort packets.
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let index1 = packets.binary_search(&div1).unwrap() + 1;
    let index2 = packets.binary_search(&div2).unwrap() + 1;

    Some(index1 * index2)
}

fn parse(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter_map(|text| (!text.is_empty()).then(|| parse_packet(text)))
        .collect::<Vec<Packet>>()
}

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    // In Rust, PartialOrd is implemented for types that showcase a property
    // of partial ordering -- in the mathematical sense:
    // 1. a <= a (reflexivity)
    // 2. a <= b and b <= a => a == b (anti-symmetry)
    // 3. a <= b and b <= c => a <= c (transivitity)
    // In practice, this trait enables the <, <=, > and >= operators.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Packet {
    // In Rust, Ord is implemented for types that showcase a property
    // of total ordering -- in the mathematical sense:
    // * Partial ordering
    // * At least one of a <= b or b <= a is always true.
    // In practice, this trait enables .sort() and .binary_search(), notably.
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::List(u), Packet::List(v)) => u.cmp(v),
            (Packet::Int(_), _) => Packet::List(vec![self.clone()]).cmp(other),
            (_, Packet::Int(_)) => self.cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

fn parse_packet(text: &str) -> Packet {
    if text.chars().nth(0).unwrap() == '[' {
        let mut stack_level = 0;
        let left_bracket = 1;
        let right_bracket = text.len() - 1;

        Packet::List(
            text[left_bracket..right_bracket]
                .split(|c| {
                    // We want to split LHS and RHS on
                    //     +- this comma, so we split here (at stack level 0), then
                    //     v  we consume (don't split) the rest of the RHS.
                    // [..., [..., ...]]
                    //       :--------:
                    //            +-- This is parsed in recursive parse_packet() calls.
                    if c == '[' {
                        stack_level += 1; // push
                    } else if c == ']' {
                        stack_level -= 1; // pop
                    }
                    c == ',' && stack_level == 0
                })
                .filter(|s| !s.is_empty()) // Maybe '[]'
                .map(|s| parse_packet(s))
                .collect(),
        )
    } else {
        Packet::Int(text.parse::<u32>().unwrap())
    }
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 13);
    assert_eq!(part1(input), Some(5340));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 13);
    assert_eq!(part2(input), Some(21276));
}
