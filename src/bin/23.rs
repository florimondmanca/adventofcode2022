use std::collections::HashMap;

// Unstable Diffusion
// https://adventofcode.com/2022/day/23

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<usize> {
    let mut board = parse(input);

    let mut first = 0;
    for _ in 1..=10 {
        round(&mut board, first);
        first = (first + 1) % 4;
    }

    let mut elve_rows = Vec::new();
    let mut elve_cols = Vec::new();

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] == Tile::Elf {
                elve_rows.push(row);
                elve_cols.push(col);
            }
        }
    }

    let min_row = *elve_rows.iter().min().unwrap();
    let max_row = *elve_rows.iter().max().unwrap();
    let min_col = *elve_cols.iter().min().unwrap();
    let max_col = *elve_cols.iter().max().unwrap();

    let mut total_ground = 0;

    for row in min_row..max_row + 1 {
        for col in min_col..max_col + 1 {
            if board[row][col] == Tile::Ground {
                total_ground += 1;
            }
        }
    }

    Some(total_ground)
}

fn part2(input: &str) -> Option<u32> {
    let mut board = parse(input);

    let mut first = 0;
    let mut round_number = 1;
    loop {
        let num_moved = round(&mut board, first);
        if num_moved == 0 {
            break;
        }
        first = (first + 1) % 4;
        round_number += 1;
    }

    Some(round_number)
}

fn neighbors(board: &Board, row: usize, col: usize) -> [bool; 4] {
    [
        // North
        board[row - 1][col - 1] == Tile::Elf
            || board[row - 1][col] == Tile::Elf
            || board[row - 1][col + 1] == Tile::Elf,
        // South
        board[row + 1][col - 1] == Tile::Elf
            || board[row + 1][col] == Tile::Elf
            || board[row + 1][col + 1] == Tile::Elf,
        // West
        board[row - 1][col - 1] == Tile::Elf
            || board[row][col - 1] == Tile::Elf
            || board[row + 1][col - 1] == Tile::Elf,
        // East
        board[row - 1][col + 1] == Tile::Elf
            || board[row][col + 1] == Tile::Elf
            || board[row + 1][col + 1] == Tile::Elf,
    ]
}

fn round(board: &mut Board, first: usize) -> usize {
    let mut next_elves: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut num_considering_moving_at: HashMap<(usize, usize), usize> = HashMap::new();

    // First half: elves propose their moves.

    let moves: Vec<(isize, isize)> = vec![
        // (row, col)
        (-1, 0), // N
        (1, 0),  // S
        (0, -1), // W
        (0, 1),  // E
    ];

    let mut num_moved = 0;

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] != Tile::Elf {
                continue;
            }

            let ns = neighbors(&board, row, col);

            if ns.iter().all(|x| !x) {
                continue;
            }

            for direction in 0..4 {
                let direction = (first + direction) % 4;
                if !ns[direction] {
                    let (drow, dcol) = moves[direction];
                    let nrow = (row as isize + drow) as usize;
                    let ncol = (col as isize + dcol) as usize;
                    next_elves.insert((row, col), (nrow, ncol));
                    *num_considering_moving_at.entry((nrow, ncol)).or_insert(0) += 1;
                    break;
                }
            }
        }
    }

    // Second half: elves only move if they were the only elf to plan to move to their position.

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] != Tile::Elf {
                continue;
            }

            match next_elves.get(&(row, col)) {
                Some(&(nrow, ncol)) => {
                    match num_considering_moving_at.get(&(nrow, ncol)) {
                        Some(1) => {
                            num_moved += 1;
                            board[row][col] = Tile::Ground;
                            board[nrow][ncol] = Tile::Elf;
                        }
                        _ => {}
                    };
                }
                None => {}
            };
        }
    }

    num_moved
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Elf,
    Ground,
}

type Board = Vec<Vec<Tile>>;

fn parse(input: &str) -> Board {
    let mut board: Board = Vec::new();

    let numrows = input.lines().count();

    for line in input.lines() {
        let numcols = line.len();

        let mut tiles = Vec::new();

        // Left padding
        for _ in 0..numcols {
            tiles.push(Tile::Ground);
        }

        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Elf,
                '.' => Tile::Ground,
                _ => unreachable!(),
            };
            tiles.push(tile);
        }

        // Right padding
        for _ in 0..numcols {
            tiles.push(Tile::Ground);
        }

        board.push(tiles);
    }

    for _ in 0..numrows {
        // Bottom padding
        board.push(vec![Tile::Ground; board[0].len()]);
        // Top padding
        board.insert(0, vec![Tile::Ground; board[0].len()]);
    }

    board
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 23);
    assert_eq!(part1(input), Some(3925));
}

#[test]
#[ignore = "slow"]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 23);
    assert_eq!(part2(input), Some(903));
}
