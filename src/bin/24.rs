// Blizzard Basin
// https://adventofcode.com/2022/day/24

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<u32> {
    let board = Board::from(input);

    let (steps, _) = solve(
        board.start,
        board.destination,
        &mut HashMap::from([(0, board.clone())]),
    );

    Some(steps)
}

fn part2(input: &str) -> Option<u32> {
    let board = Board::from(input);

    let (steps1, board) = solve(
        board.start,
        board.destination,
        &mut HashMap::from([(0, board.clone())]),
    );

    let (steps2, board) = solve(
        board.destination,
        board.start,
        &mut HashMap::from([(0, board.clone())]),
    );

    let (steps3, _) = solve(
        board.start,
        board.destination,
        &mut HashMap::from([(0, board.clone())]),
    );

    Some(steps1 + steps2 + steps3)
}

type Point2D = (usize, usize);

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Blizzards = HashMap<Point2D, Vec<Direction>>;

#[derive(Clone)]
struct Board {
    width: usize,
    height: usize,
    blizzards: Blizzards,
    start: Point2D,
    destination: Point2D,
}

impl Board {
    fn new(width: usize, height: usize, blizzards: Blizzards) -> Self {
        Self {
            width,
            height,
            blizzards,
            start: (0, 1),
            destination: (height - 1, width - 2),
        }
    }

    fn is_in_bounds(&self, pos: &Point2D) -> bool {
        let (row, col) = *pos;
        row > 0 && col > 0 && row < self.height - 1 && col < self.width - 1
    }

    fn is_ground(&self, pos: &Point2D) -> bool {
        !self.blizzards.contains_key(&pos)
    }

    fn get_neighbors(&self, pos: &Point2D) -> Vec<Point2D> {
        let mut neighbors = Vec::new();

        let (row, col) = *pos;

        if row + 1 < self.height {
            neighbors.push((row + 1, col));
        }

        if row > 0 {
            neighbors.push((row - 1, col));
        }

        if col + 1 < self.width {
            neighbors.push((row, col + 1));
        }

        if col > 0 {
            neighbors.push((row, col - 1));
        }

        neighbors
    }

    fn next(&self) -> Board {
        let mut blizzards: Blizzards = HashMap::new();

        for ((row, col), directions) in &self.blizzards {
            let row = *row;
            let col = *col;
            for d in directions {
                let (mut nrow, mut ncol) = match d {
                    Direction::Up => (row - 1, col),
                    Direction::Right => (row, col + 1),
                    Direction::Down => (row + 1, col),
                    Direction::Left => (row, (col + self.width - 1) % self.width),
                };

                nrow = match nrow {
                    0 => self.height - 2,
                    r if r == self.height - 1 => 1,
                    r => r,
                };

                ncol = match ncol {
                    0 => self.width - 2,
                    c if c == self.width - 1 => 1,
                    c => c,
                };

                let pos = (nrow % (self.height - 1), ncol % (self.width - 1));

                blizzards.entry(pos).or_insert(Vec::new()).push(d.clone());
            }
        }

        Board::new(self.width, self.height, blizzards)
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().nth(0).unwrap().len();

        let mut blizzards = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let direction = match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => continue,
                };

                blizzards.insert((row, col), vec![direction]);
            }
        }

        Self::new(width, height, blizzards)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Visit(u32, Point2D);

impl Visit {
    fn new(steps: u32, pos: Point2D) -> Self {
        Self(steps, pos)
    }
}

fn solve(start: Point2D, destination: Point2D, boards: &mut HashMap<u32, Board>) -> (u32, Board) {
    // Breadth-first search (BFS) through the changing board.
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(Visit::new(0, start));

    loop {
        let v = queue.pop_front().expect("No path to destination");

        if !seen.insert(v.clone()) {
            continue;
        }

        let Visit(steps, pos) = v;

        let next_steps = steps + 1;

        if !boards.contains_key(&next_steps) {
            boards.insert(next_steps, boards.get(&steps).unwrap().next());
        }

        let next_board = boards.get(&next_steps).unwrap();

        // Maybe we can reach the destination if we stay here and wait for
        // blizzards to move some more.
        if next_board.is_ground(&pos) {
            queue.push_back(Visit::new(next_steps, pos.clone()));
        }

        let neighbors = next_board.get_neighbors(&pos);

        if neighbors.iter().any(|&p| p == destination) {
            // We're just a step away from the destination.
            return (next_steps, next_board.clone());
        }

        // Maybe we can reach the destination through one of the neighboring ground tiles.
        for p in neighbors {
            if p == start || (next_board.is_in_bounds(&p) && next_board.is_ground(&p)) {
                queue.push_back(Visit::new(next_steps, p));
            }
        }
    }
}

#[test]
#[ignore = "slow"]
fn test_24_part1() {
    let input = &advent_of_code::read_file("inputs", 24);
    assert_eq!(part1(input), Some(343));
}

#[test]
fn test_24_part1_example() {
    let input = &advent_of_code::read_file("examples", 24);
    assert_eq!(part1(input), Some(18));
}

#[test]
#[ignore = "slow"]
fn test_24_part2() {
    let input = &advent_of_code::read_file("inputs", 24);
    assert_eq!(part2(input), Some(960));
}

#[test]
fn test_24_part2_example() {
    let input = &advent_of_code::read_file("examples", 24);
    assert_eq!(part2(input), Some(54));
}
