use regex::Regex;
use std::{collections::HashMap, ops::Range};

pub fn run() {
    let example = include_str!("inputs/15.example.txt");
    let map = parse(example);
    map.show();
    println!("Example (Part 1): {}", map.count_excluded(10));

    let content = include_str!("inputs/15.txt");
    let map = parse(content);
    println!("Answer (Part 1): {}", map.count_excluded(200000));
}

type Node = (i32, i32);
type Cells = HashMap<Node, char>;

struct Map {
    cells: Cells,
    xs: Range<i32>,
    ys: Range<i32>,
}

impl Map {
    fn new(cells: Cells, xs: Range<i32>, ys: Range<i32>) -> Self {
        Self { cells, xs, ys }
    }

    fn show(&self) {
        for y in self.ys.clone() {
            print!("{y:<4} ");
            for x in self.xs.clone() {
                match self.cells.get(&(x, y)) {
                    Some(c) => print!("{}", c),
                    None => print!("."),
                }
            }
            println!();
        }
    }

    fn count_excluded(&self, y: i32) -> usize {
        self.xs
            .clone()
            .map(|x| *self.cells.get(&(x, y)).unwrap_or(&'.'))
            .filter(|&c| c == '#')
            .count()
    }
}

fn parse(content: &str) -> Map {
    let mut cells: Cells = HashMap::new();

    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut xs = 0..0;
    let mut ys = 0..0;

    for line in content.lines() {
        let cap = re.captures(line).unwrap();

        let x_s = cap[1].parse::<i32>().unwrap();
        let y_s = cap[2].parse::<i32>().unwrap();
        let x_b = cap[3].parse::<i32>().unwrap();
        let y_b = cap[4].parse::<i32>().unwrap();

        for x in vec![x_s, x_b] {
            if x < xs.start {
                xs = x..xs.end;
            }
            if x > xs.end {
                xs = xs.start..x;
            }
        }

        for y in vec![y_s, y_b] {
            if y < ys.start {
                ys = y..ys.end;
            }
            if y > ys.end {
                ys = ys.start..y;
            }
        }

        cells.insert((x_s, y_s), 'S');
        cells.insert((x_b, y_b), 'B');

        // All locations within this radius of S cannot contain a beacon.
        let radius = (x_b - x_s).abs() + (y_b - y_s).abs();

        for dy in -radius..radius + 1 {
            let y = y_s + dy;
            let dx = radius - dy.abs();
            for x in x_s - dx..x_s + dx + 1 {
                if !cells.contains_key(&(x, y)) {
                    cells.insert((x, y), '#');
                }
            }
        }
    }

    Map::new(cells, xs, ys)
}
