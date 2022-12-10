use std::collections::HashSet;

use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/9.txt");
    let moves = parse(content);

    let knots = vec![(0, 0), (0, 0)];
    println!("Answer (part 1): {}", solve(knots.clone(), &moves));

    let knots = (0..10).map(|_| (0, 0)).collect::<Vec<Knot>>();
    println!("Answer (part 2): {}", solve(knots.clone(), &moves));
}

fn solve(knots: Vec<Knot>, moves: &Vec<Move>) -> usize {
    let mut rope = Rope::new(knots);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope.tail());

    for mv in moves {
        for _ in 0..mv.quantity {
            rope.move_towards(&mv.direction.as_str());
            visited.insert(rope.tail());
        }
    }

    visited.len()
}

type Knot = (i32, i32);

fn moved(knot: Knot, displacement: (i32, i32)) -> Knot {
    (knot.0 + displacement.0, knot.1 + displacement.1)
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(knots: Vec<Knot>) -> Self {
        Self { knots }
    }

    fn tail(&self) -> Knot {
        self.knots.last().unwrap().clone()
    }

    fn move_towards(&mut self, direction: &str) {
        let mut new_knots: Vec<Knot> = Vec::new();

        let mut head = self.knots.first().unwrap().clone();

        // The rope head moves according to commands.
        let new_head = match direction {
            "U" => moved(head, (0, 1)),
            "R" => moved(head, (1, 0)),
            "D" => moved(head, (0, -1)),
            "L" => moved(head, (-1, 0)),
            _ => panic!("unknown direction"),
        };
        new_knots.push(new_head);

        // Each other knot catches up on the knot before.
        head = new_head;

        for knot in self.knots.iter().skip(1) {
            let knot = knot.clone();
            let new_knot = catch_up(knot, head);
            new_knots.push(new_knot);
            head = new_knot;
        }

        assert!(self.knots.len() == new_knots.len());

        self.knots = new_knots;
    }
}

fn catch_up(tail: Knot, head: Knot) -> Knot {
    let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

    let displacement = match (dx, dy) {
        (0, 0) => (0, 0),
        (0, 1) => (0, 0),
        (0, 2) => (0, 1),
        (0, -1) => (0, 0),
        (0, -2) => (0, -1),
        (1, 0) => (0, 0),
        (1, 1) => (0, 0),
        (1, 2) => (1, 1),
        (1, -1) => (0, 0),
        (1, -2) => (1, -1),
        (2, 0) => (1, 0),
        (2, 1) => (1, 1),
        (2, 2) => (1, 1),
        (2, -1) => (1, -1),
        (2, -2) => (1, -1),
        (-1, 0) => (0, 0),
        (-1, 1) => (0, 0),
        (-1, 2) => (-1, 1),
        (-1, -1) => (0, 0),
        (-1, -2) => (-1, -1),
        (-2, 0) => (-1, 0),
        (-2, 1) => (-1, 1),
        (-2, 2) => (-1, 1),
        (-2, -1) => (-1, -1),
        (-2, -2) => (-1, -1),
        _ => panic!("unknown transition: ({dx}, {dy})"),
    };

    moved(tail, displacement)
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
