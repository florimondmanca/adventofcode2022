use std::ops::Add;

// Monkey Map
// https://adventofcode.com/2022/day/22

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: usize,
    y: usize,
}

impl Vec2D {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn part1(input: &str) -> Option<usize> {
    let (map, commands) = parse(input);

    let mut pos = Vec2D::new(map[1].iter().position(|i| *i == Tile::Open).unwrap(), 1);
    let mut direction = RIGHT;

    for command in commands {
        match command {
            Command::MoveForward(n) => {
                for _ in 0..n {
                    let mut new_pos = match direction {
                        RIGHT => Vec2D::new(pos.x + 1, pos.y),
                        DOWN => Vec2D::new(pos.x, pos.y + 1),
                        LEFT => Vec2D::new(pos.x - 1, pos.y),
                        _ => Vec2D::new(pos.x, pos.y - 1),
                    };

                    if map[new_pos.y][new_pos.x] == Tile::Nothing {
                        match direction {
                            RIGHT => {
                                new_pos.x = map[pos.y]
                                    .iter()
                                    .position(|tile| *tile != Tile::Nothing)
                                    .unwrap();
                            }
                            DOWN => {
                                new_pos.y = map
                                    .iter()
                                    .position(|row| row[pos.x] != Tile::Nothing)
                                    .unwrap();
                            }
                            LEFT => {
                                new_pos.x = map[pos.y].len()
                                    - 1
                                    - map[pos.y]
                                        .iter()
                                        .rev()
                                        .position(|tile| *tile != Tile::Nothing)
                                        .unwrap();
                            }
                            _ => {
                                new_pos.y = map.len()
                                    - 1
                                    - map
                                        .iter()
                                        .rev()
                                        .position(|row| row[pos.x] != Tile::Nothing)
                                        .unwrap();
                            }
                        }
                    }

                    if map[new_pos.y][new_pos.x] == Tile::Open {
                        pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
            Command::TurnLeft => {
                direction = match direction {
                    UP => LEFT,
                    LEFT => DOWN,
                    DOWN => RIGHT,
                    RIGHT => UP,
                    _ => unreachable!(),
                };
            }
            Command::TurnRight => {
                direction = match direction {
                    UP => RIGHT,
                    RIGHT => DOWN,
                    DOWN => LEFT,
                    LEFT => UP,
                    _ => unreachable!(),
                };
            }
        }
    }

    Some(pos.y * 1000 + 4 * pos.x + direction)
}

fn part2(_input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Nothing,
    Open,
    Wall,
}

type Map = Vec<Vec<Tile>>;

#[derive(Debug)]
enum Command {
    MoveForward(i32),
    TurnLeft,
    TurnRight,
}

fn parse(input: &str) -> (Map, Vec<Command>) {
    let (map_input, commands_input) = input.split_once("\n\n").unwrap();

    let mut map: Map = Vec::new();
    let mut max = 0;

    for line in map_input.lines() {
        let mut row = Vec::new();

        // Left padding
        row.push(Tile::Nothing);

        max = max.max(line.len());

        for c in line.chars() {
            row.push(match c {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                ' ' => Tile::Nothing,
                _ => unreachable!(),
            });
        }

        // Fill remaining width.
        row.extend(vec![Tile::Nothing; max - line.len()]);

        // Right padding
        row.push(Tile::Nothing);

        map.push(row);
    }

    // Top padding
    map.insert(0, vec![Tile::Nothing; map[0].len()]);

    // Bottom padding
    map.push(vec![Tile::Nothing; map[0].len()]);

    let instructions = commands_input.chars().collect::<Vec<_>>();
    let mut commands = Vec::new();
    let mut start = 0;
    let mut current = 0;

    loop {
        while current < instructions.len() && instructions[current].is_digit(10) {
            current += 1;
        }

        let num_steps = instructions[start..current]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        commands.push(Command::MoveForward(num_steps));

        if current == instructions.len() {
            break;
        }

        match instructions[current] {
            'L' => commands.push(Command::TurnLeft),
            'R' => commands.push(Command::TurnRight),
            _ => break,
        }

        current += 1;
        start = current;
    }

    (map, commands)
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 22);
    assert_eq!(part1(input), Some(88268));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 22);
    assert_eq!(part2(input), None);
}
