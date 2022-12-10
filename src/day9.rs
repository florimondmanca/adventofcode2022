use std::collections::HashSet;

use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/9.txt");
    let moves = parse(content);

    let mut rope = Rope::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope.tail.clone());

    for mv in moves {
        for _ in 0..mv.quantity {
            rope.apply(&mv.direction.as_str());
            visited.insert(rope.tail.clone());
        }
    }

    println!("Answer (part 1): {}", visited.len());
}

fn moved(point: (i32, i32), d: (i32, i32)) -> (i32, i32) {
    (point.0 + d.0, point.1 + d.1)
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

fn catch_up(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (0, 0) => (0, 0),
        (0, 1) => (0, 0),
        (0, 2) => (0, 1),
        (0, -1) => (0, 0),
        (0, -2) => (0, -1),
        //
        (1, 0) => (0, 0),
        (1, 1) => (0, 0),
        (1, 2) => (1, 1),
        (1, -1) => (0, 0),
        (1, -2) => (1, -1),
        //
        (2, 0) => (1, 0),
        (2, 1) => (1, 1),
        (2, -1) => (1, -1),
        //
        (-1, 0) => (0, 0),
        (-1, 1) => (0, 0),
        (-1, 2) => (-1, 1),
        (-1, -1) => (0, 0),
        (-1, -2) => (-1, -1),
        //
        (-2, 0) => (-1, 0),
        (-2, 1) => (-1, 1),
        (-2, -1) => (-1, -1),
        _ => panic!("stretched too far"),
    }
}

impl Rope {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn apply(&mut self, direction: &str) {
        let new_head = match direction {
            "U" => moved(self.head, (0, 1)),
            "R" => moved(self.head, (1, 0)),
            "D" => moved(self.head, (0, -1)),
            "L" => moved(self.head, (-1, 0)),
            _ => panic!("unknown direction"),
        };

        let new_tail = moved(self.tail, catch_up(self.tail, new_head));

        self.head = new_head;
        self.tail = new_tail;
    }
}

struct Move {
    direction: String,
    quantity: u32,
}

impl Move {
    fn new(direction: String, quantity: u32) -> Self {
        Self {
            direction,
            quantity,
        }
    }
}

fn parse(content: &str) -> Vec<Move> {
    content
        .lines()
        .map(|line| {
            let (direction, quantity) = line.split(" ").collect_tuple().unwrap();
            let quantity = quantity.parse::<u32>().unwrap();
            Move::new(String::from(direction), quantity)
        })
        .collect()
}
