use std::collections::HashMap;
use std::collections::HashSet;

use take_until::TakeUntilExt;

pub fn run() {
    let content = include_str!("inputs/08.txt");
    let grid = parse(content);
    let count = count_visible(&grid);
    println!("Answer (part 1): {count}");

    let score = maximize_scenic_score(&grid);
    println!("Answer (part 2): {score}");
}

type GridMap = HashMap<(usize, usize), u32>;

struct Grid {
    map: GridMap,
    size: usize,
}

impl Grid {
    fn new(map: GridMap, size: usize) -> Self {
        Self { map, size }
    }

    fn get(&self, row: usize, col: usize) -> u32 {
        self.map.get(&(row, col)).unwrap().clone()
    }
}

fn parse(content: &str) -> Grid {
    let mut map: GridMap = HashMap::new();
    let size = content.lines().count();

    for (i, line) in content.lines().enumerate() {
        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .for_each(|(j, h)| {
                map.insert((i, j), h);
            });
    }

    return Grid::new(map, size);
}

fn count_visible(grid: &Grid) -> u32 {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let n = grid.size;

    for row in 0..n {
        for col in 0..n {
            let h = grid.get(row, col);

            let vleft = (0..col).map(|c| grid.get(row, c)).all(|hp| hp < h);
            if vleft {
                visible.insert((row, col));
            }

            let vtop = (0..row).map(|r| grid.get(r, col)).all(|hp| hp < h);
            if vtop {
                visible.insert((row, col));
            }

            let row = n - 1 - row;
            let col = n - 1 - col;

            let h = grid.get(row, col);

            let vright = (col + 1..n).map(|c| grid.get(row, c)).all(|hp| hp < h);
            if vright {
                visible.insert((row, col));
            }

            let vbottom = (row + 1..n).map(|r| grid.get(r, col)).all(|hp| hp < h);
            if vbottom {
                visible.insert((row, col));
            }
        }
    }

    return visible.len() as u32;
}

fn maximize_scenic_score(grid: &Grid) -> u32 {
    let n = grid.size;
    let mut highest = 0;

    for row in 0..n {
        for col in 0..n {
            let h = grid.get(row, col);

            let score = vec![
                // Up
                (0..row)
                    .rev()
                    .map(|r| grid.get(r, col))
                    .take_until(|&hp| hp >= h)
                    .count(),
                // Down
                (row + 1..n)
                    .map(|r| grid.get(r, col))
                    .take_until(|&hp| hp >= h)
                    .count(),
                // Left
                (0..col)
                    .rev()
                    .map(|c| grid.get(row, c))
                    .into_iter()
                    .take_until(|&hp| hp >= h)
                    .count(),
                // Right
                (col + 1..n)
                    .map(|c| grid.get(row, c))
                    .take_until(|&hp| hp >= h)
                    .count(),
            ]
            .into_iter()
            .product::<usize>() as u32;

            if score > highest {
                highest = score;
            }
        }
    }

    highest
}