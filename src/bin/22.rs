use std::ops::{Add, Mul, Range};

fn main() {
    println!("Monkey Map");
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i32> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

fn part1(input: &str) -> Option<i32> {
    let (map, commands) = parse(input);

    let mut pos = Vec2D::new(
        (map[0].range.start + map[0].tiles.iter().position(|&x| x == '.').unwrap()) as i32,
        0,
    );

    let mut direction = 0;

    for command in commands {
        match command {
            Command::MoveForward(n) => {
                for _ in 0..n {
                    let mut new_pos = pos
                        + match direction {
                            0 => Vec2D::new(1, 0),
                            1 => Vec2D::new(0, 1),
                            2 => Vec2D::new(-1, 0),
                            3 => Vec2D::new(0, -1),
                            _ => panic!(),
                        };

                    // TODO: these y positions may not be accessible

                    let ymax = (0..map.len())
                        .rev()
                        .filter(|&y| map[y].range.contains(&(pos.x as usize)))
                        .nth(0)
                        .unwrap() as i32;

                    let ymin = (0..map.len())
                        .filter(|&y| map[y].range.contains(&(pos.x as usize)))
                        .nth(0)
                        .unwrap() as i32;

                    if new_pos.y < ymin {
                        new_pos.y = ymax;
                    }

                    if new_pos.y >= ymax {
                        new_pos.y = ymin;
                    }

                    let row = &map[new_pos.y as usize];

                    if new_pos.x < row.range.start as i32 {
                        new_pos.x = row.range.end as i32;
                    }

                    if new_pos.x >= row.range.end as i32 {
                        new_pos.x = row.range.start as i32;
                    }

                    let next_tile = row.tiles[new_pos.x as usize - row.range.start];

                    if next_tile == '#' {
                        break;
                    }

                    pos = new_pos;
                }
            }
            Command::TurnLeft => {
                direction = (((direction - 1) % 4) + 4) % 4;
            }
            Command::TurnRight => {
                direction = (direction + 1) % 4;
            }
        }
    }

    Some((pos.y + 1) * 1000 + 4 * (pos.x + 1) + direction)
}

fn part2(_input: &str) -> Option<u32> {
    None
}

struct Row {
    range: Range<usize>,
    tiles: Vec<char>,
}

type Map = Vec<Row>;

#[derive(Debug)]
enum Command {
    MoveForward(i32),
    TurnLeft,
    TurnRight,
}

fn parse(input: &str) -> (Map, Vec<Command>) {
    let mut rows: Vec<Row> = Vec::new();

    let num_lines = input.lines().count();

    for line in input.lines().take(num_lines - 2) {
        let start = line.chars().take_while(|&c| c == ' ').count();
        let tiles = line.replace(" ", "").chars().collect::<Vec<_>>();
        let range = start..start + tiles.len();
        rows.push(Row { range, tiles });
    }

    let mut commands = Vec::new();

    let instructions = input.lines().last().unwrap().chars().collect::<Vec<_>>();

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

    (rows, commands)
}
