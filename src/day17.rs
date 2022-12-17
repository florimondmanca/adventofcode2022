use std::{collections::HashSet, ops::Range};

pub fn run() {
    let content = include_str!("inputs/17.example.txt");

    let jets = parse(content);
    let shapes = get_shapes();

    let mut game = Game::new(0..7, &shapes, &jets);
    game.run(10);
    let rock = game.spawn_rock();
    game.show(&rock);

    let content = include_str!("inputs/17.txt");
    let jets = parse(content);

    println!(
        "Answer (Part 1): {}",
        Game::new(0..7, &shapes, &jets).run(2022)
    );
}

fn parse(content: &str) -> Vec<char> {
    content.trim().chars().collect()
}

fn get_shapes() -> Vec<Shape> {
    let minus = Shape::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)], Rect::new(0, 0, 4, 1));
    let plus = Shape::new(
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        Rect::new(0, 0, 3, 3),
    );
    let inv_l = Shape::new(
        vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        Rect::new(0, 0, 3, 3),
    );
    let i = Shape::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)], Rect::new(0, 0, 1, 4));
    let square = Shape::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)], Rect::new(0, 0, 2, 2));

    vec![minus, plus, inv_l, i, square]
}

type Node = (i32, i32);

#[derive(Debug, Clone)]
struct Rect {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn new(top: i32, left: i32, width: i32, height: i32) -> Self {
        let right = left + width;
        let bottom = top - height;
        Self {
            top,
            bottom,
            left,
            right,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
struct Shape {
    nodes: Vec<Node>,
    rect: Rect,
}

impl Shape {
    fn new(nodes: Vec<Node>, rect: Rect) -> Self {
        Self { nodes, rect }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    rect: Rect,
    shape: Shape,
    positions: HashSet<Node>,
}

impl Rock {
    fn new(pos: Node, shape: Shape) -> Self {
        let (left, top) = pos;

        let positions = shape
            .nodes
            .iter()
            .map(|(x, y)| (left + x, top - 1 - y))
            .collect();

        let width = shape.rect.width;
        let height = shape.rect.height;
        let rect = Rect::new(top, left, width, height);

        Self {
            rect,
            shape,
            positions,
        }
    }
}

struct Game {
    well: Range<i32>,
    map: HashSet<(i32, i32)>,
    shapes: Box<dyn Iterator<Item = Shape>>,
    jets: Box<dyn Iterator<Item = char>>,
}

impl Game {
    fn new(well: Range<i32>, shapes: &Vec<Shape>, jets: &Vec<char>) -> Self {
        Self {
            well,
            map: HashSet::new(),
            shapes: Box::new(shapes.clone().into_iter().cycle()),
            jets: Box::new(jets.clone().into_iter().cycle()),
        }
    }

    fn spawn_rock(&mut self) -> Rock {
        let shape = self.shapes.next().unwrap();

        let x = self.well.start + 2;
        let y = self.get_height() + 3 + shape.rect.height;

        Rock::new((x, y), shape)
    }

    fn next_jet(&mut self) -> char {
        self.jets.next().unwrap()
    }

    fn intersects(&self, rock: &Rock) -> bool {
        rock.positions.iter().any(|&pos| self.map.contains(&pos))
    }

    fn push_left(&self, rock: &Rock) -> Option<Rock> {
        let pushed_rock = Rock::new((rock.rect.left - 1, rock.rect.top), rock.shape.clone());

        if pushed_rock.rect.left == self.well.start - 1 {
            return None;
        }

        if self.intersects(&pushed_rock) {
            return None;
        }

        Some(pushed_rock)
    }

    fn push_right(&self, rock: &Rock) -> Option<Rock> {
        let pushed_rock = Rock::new((rock.rect.left + 1, rock.rect.top), rock.shape.clone());

        if pushed_rock.rect.right == self.well.end + 1 {
            return None;
        }

        if self.intersects(&pushed_rock) {
            return None;
        }

        Some(pushed_rock)
    }

    fn fall(&self, rock: &Rock) -> Option<Rock> {
        let fallen_rock = Rock::new((rock.rect.left, rock.rect.top - 1), rock.shape.clone());

        if fallen_rock.rect.bottom < 0 {
            return None;
        }

        if self.intersects(&fallen_rock) {
            return None;
        }

        Some(fallen_rock)
    }

    fn stop(&mut self, rock: Rock) {
        for pos in rock.positions.iter() {
            self.map.insert(*pos);
        }
    }

    fn show(&self, rock: &Rock) {
        let top = self.get_height().max(rock.rect.top);

        for y in (0..top).rev() {
            print!("|");

            for col in self.well.clone() {
                let x = col;

                if self.map.contains(&(x, y)) {
                    print!("#");
                } else if rock.positions.contains(&(x, y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }

            println!("|");
        }

        print!("+");
        for _ in self.well.clone() {
            print!("-");
        }
        println!("+");
    }

    fn get_height(&self) -> i32 {
        self.map.iter().map(|(_, y)| *y + 1).max().unwrap_or(0)
    }

    fn run(&mut self, num_rocks: u32) -> i32 {
        let mut num_stopped_rocks = 0;

        while num_stopped_rocks < num_rocks {
            let mut rock = self.spawn_rock();

            loop {
                let jet = self.next_jet();

                match match jet {
                    '<' => self.push_left(&rock),
                    '>' => self.push_right(&rock),
                    _ => panic!("unexpected character"),
                } {
                    Some(r) => rock = r,
                    None => {}
                };

                match self.fall(&rock) {
                    None => {
                        self.stop(rock);
                        num_stopped_rocks += 1;
                        break;
                    }
                    Some(r) => {
                        rock = r;
                    }
                };
            }
        }

        self.get_height()
    }
}
