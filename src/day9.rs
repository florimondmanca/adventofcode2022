use std::collections::HashSet;

use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/9.txt");
    let moves = parse(content);

    let rope = vec![(0, 0), (0, 0)];
    println!("Answer (part 1): {}", solve(rope.clone(), &moves));

    let rope = (0..10).map(|_| (0, 0)).collect::<Vec<Knot>>();
    println!("Answer (part 2): {}", solve(rope.clone(), &moves));
}

fn solve(mut rope: Rope, moves: &Vec<Move>) -> usize {
    let mut visited: HashSet<Knot> = HashSet::new();
    visited.insert(tail(&rope));

    for (direction, quantity) in moves {
        for _ in 0..*quantity {
            rope = move_towards(direction.as_str(), &rope);
            visited.insert(tail(&rope));
        }
    }

    visited.len()
}

type Knot = (i32, i32);
type Rope = Vec<Knot>;
type Move = (String, u32);

fn parse(content: &str) -> Vec<Move> {
    content
        .lines()
        .map(|line| {
            let (direction, quantity) = line.split(" ").collect_tuple().unwrap();
            let quantity = quantity.parse::<u32>().unwrap();
            (String::from(direction), quantity)
        })
        .collect()
}

fn moved(knot: Knot, displacement: (i32, i32)) -> Knot {
    (knot.0 + displacement.0, knot.1 + displacement.1)
}

fn tail(rope: &Rope) -> Knot {
    rope.last().unwrap().clone()
}

fn move_towards(direction: &str, rope: &Rope) -> Rope {
    let mut moved_rope: Rope = vec![];

    // The rope head moves according to commands.
    let mut head = rope.first().unwrap().clone();
    let moved_head = match direction {
        "U" => moved(head, (0, 1)),
        "R" => moved(head, (1, 0)),
        "D" => moved(head, (0, -1)),
        "L" => moved(head, (-1, 0)),
        _ => panic!("unknown direction"),
    };
    moved_rope.push(moved_head);

    // Other knots catch up on the knot before them.
    head = moved_head;
    for knot in rope.iter().skip(1) {
        let knot = knot.clone();
        let moved_knot = catch_up(knot, head);
        moved_rope.push(moved_knot);
        head = moved_knot;
    }
    assert!(moved_rope.len() == rope.len());

    moved_rope
}

fn catch_up(tail: Knot, head: Knot) -> Knot {
    let gap_x = head.0 - tail.0;
    let gap_y = head.1 - tail.1;
    let dist = gap_x.abs() + gap_y.abs();

    let dx = if gap_x.abs() <= 1 && dist <= 2 {
        0
    } else {
        gap_x.signum()
    };

    let dy = if gap_y.abs() <= 1 && dist <= 2 {
        0
    } else {
        gap_y.signum()
    };

    moved(tail, (dx, dy))
}
