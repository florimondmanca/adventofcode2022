use std::cmp::Ordering;

use itertools::Itertools;

// Kudos to: https://www.reddit.com/r/adventofcode/comments/zkmyh4/comment/j01mqo7/

pub fn run() {
    let content = include_str!("inputs/13.txt");

    let mut packets = content
        .lines()
        .filter_map(|text| (!text.is_empty()).then(|| parse(text)))
        .collect::<Vec<Packet>>();

    let mut sum = 0;

    for (i, (p1, p2)) in packets.iter().tuples().enumerate() {
        if p1 <= p2 {
            sum += i + 1;
        }
    }

    println!("Answer (Part 1): {}", sum);

    // Insert the dividers, then sort packets.
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let index1 = packets.binary_search(&div1).unwrap() + 1;
    let index2 = packets.binary_search(&div2).unwrap() + 1;
    let mult = index1 * index2;

    println!("Answer (Part 2): {}", mult);
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

fn parse(text: &str) -> Packet {
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
                    //            +-- This is parsed in recursive parse() calls.
                    if c == '[' {
                        stack_level += 1; // push
                    } else if c == ']' {
                        stack_level -= 1; // pop
                    }
                    c == ',' && stack_level == 0
                })
                .filter(|s| !s.is_empty()) // Maybe '[]'
                .map(|s| parse(s))
                .collect(),
        )
    } else {
        Packet::Int(text.parse::<u32>().unwrap())
    }
}
