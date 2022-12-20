use itertools::Itertools;
use std::{collections::HashMap, ops::Range, usize::MAX};

fn main() {
    println!("Regolith Reservoirs");
    part1_example();
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::slow!(|| {
        advent_of_code::solve!(1, part1, input);
        advent_of_code::solve!(2, part2, input);
    });
}

fn part1_example() {
    let input = &advent_of_code::read_file("examples", 14);
    let mut grid = parse(input);
    let num_grains = grid.pour_sand();
    println!("Part 1: Example: {}", num_grains);
    grid.show();
}

fn part1(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    let num_grains = grid.pour_sand();

    Some(num_grains)
}

fn part2(input: &str) -> Option<u32> {
    let example = &advent_of_code::read_file("examples", 14);
    let mut grid = parse(example);
    grid.set_floor();
    let num_grains = grid.pour_sand();
    grid.show();
    println!("Example: {}", num_grains);

    let mut grid = parse(input);
    grid.set_floor();
    let num_grains = grid.pour_sand();
    Some(num_grains)
}

type Node = (usize, usize);

struct Grid {
    cells: HashMap<Node, char>,
    xs: Range<usize>,
    ys: Range<usize>,
    floor: Option<Floor>,
}

struct Floor {
    y: usize,
    xs: Range<usize>,
}

impl Floor {
    fn show(&self) {
        let y = self.y;
        print!("{y:<5} ");
        for _ in self.xs.clone() {
            print!("#");
        }
        println!();
    }
}

impl Grid {
    fn show(&self) {
        let ys = match &self.floor {
            Some(f) => self.ys.start..f.y,
            None => self.ys.clone(),
        };

        for y in ys {
            print!("{y:<5} ");
            for x in self.xs.clone() {
                match self.cells.get(&(x, y)) {
                    Some(&c) => print!("{}", c),
                    None => print!("."),
                }
            }
            println!();
        }

        match &self.floor {
            Some(f) => f.show(),
            None => {}
        }
    }

    fn set_floor(&mut self) {
        self.floor = Some(Floor {
            y: self.ys.end + 2,
            xs: self.xs.clone(),
        });
    }

    fn pour_sand(&mut self) -> u32 {
        let mut sand = (500, 0);
        let mut num_grains = 0;

        loop {
            match self.fall(sand) {
                Some(moved_sand) => {
                    sand = moved_sand;

                    if self.floor.is_none() {
                        let fell_in_abyss = sand.1 > self.ys.end;
                        if fell_in_abyss {
                            return num_grains;
                        }
                    }
                }
                None => {
                    self.cells.insert(sand, 'o');
                    num_grains += 1;

                    if sand == (500, 0) {
                        // Blocked.
                        return num_grains;
                    }

                    sand = (500, 0);
                }
            }
        }
    }

    fn fall(&self, sand: Node) -> Option<Node> {
        let (x, y) = sand;

        match &self.floor {
            Some(floor) => {
                if y + 1 == floor.y {
                    return None;
                }
            }
            None => {}
        }

        if !self.cells.contains_key(&(x, y + 1)) {
            return Some((x, y + 1));
        }

        if !self.cells.contains_key(&(x - 1, y + 1)) {
            return Some((x - 1, y + 1));
        }

        if !self.cells.contains_key(&(x + 1, y + 1)) {
            return Some((x + 1, y + 1));
        }

        None
    }
}

fn parse(content: &str) -> Grid {
    let mut cells: HashMap<Node, char> = HashMap::new();

    let mut min_x = MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in content.lines() {
        let mut points: Vec<Node> = vec![];

        for coords in line.split(" -> ") {
            let (x, y) = coords.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

            points.push((x, y));

            if x < min_x {
                min_x = x;
            }

            if x > max_x {
                max_x = x;
            }

            if y > max_y {
                max_y = y;
            }
        }

        for (index, start) in points.iter().enumerate().take(points.len() - 1) {
            let end = points[index + 1];

            if start.0 == end.0 {
                let x = start.0;

                let (yi, yf) = vec![start.1, end.1]
                    .into_iter()
                    .sorted()
                    .collect_tuple()
                    .unwrap();

                for y in yi..yf + 1 {
                    cells.insert((x, y), '#');
                }
            } else if start.1 == end.1 {
                let y = start.1;

                let (xi, xf) = vec![start.0, end.0]
                    .into_iter()
                    .sorted()
                    .collect_tuple()
                    .unwrap();

                for x in xi..xf + 1 {
                    cells.insert((x, y), '#');
                }
            } else {
                panic!("invalid line");
            }
        }
    }

    Grid {
        cells,
        xs: min_x..max_x,
        ys: 0..max_y,
        floor: None,
    }
}
