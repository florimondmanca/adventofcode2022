use std::collections::{HashSet, VecDeque};

pub fn run() {
    let content = include_str!("inputs/18.txt");
    let cubes = parse(content);
    println!("Answer (Part 1): {}", count_total_surface_area(&cubes));
    println!("Answer (Part 2): {}", count_exterior_surface_area(&cubes));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        let mut coords = s.splitn(3, ",").map(|v| v.parse().unwrap());
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        Self::new(x, y, z)
    }

    fn neighbors(&self) -> [Self; 6] {
        [
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y, self.z + 1),
            Cube::new(self.x, self.y, self.z - 1),
        ]
    }
}

fn parse(content: &str) -> HashSet<Cube> {
    content.lines().map(Cube::from_str).collect()
}

fn count_total_surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|cube| {
            cube.neighbors()
                .iter()
                .filter(|neighbor| !cubes.contains(neighbor))
                .count()
        })
        .sum()
}

fn count_exterior_surface_area(cubes: &HashSet<Cube>) -> usize {
    // Perform a flood fill algorithm (essentially BFS),
    // starting from an outside corner of the droplet bounding box.

    let start = Cube::new(-1, -1, -1);
    let end = make_end(&cubes);

    let xs = start.x..end.x + 1;
    let ys = start.y..end.y + 1;
    let zs = start.z..end.z + 1;

    let mut exterior_area = 0;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(cube) = queue.pop_front() {
        if visited.contains(&cube) {
            continue;
        }

        let is_outside_bounding_box =
            !xs.contains(&cube.x) || !ys.contains(&cube.y) || !zs.contains(&cube.z);

        if is_outside_bounding_box {
            continue;
        }

        visited.insert(cube);

        for neighbor in cube.neighbors() {
            if cubes.contains(&neighbor) {
                exterior_area += 1;
            } else {
                queue.push_back(neighbor);
            }
        }
    }

    exterior_area
}

fn make_end(cubes: &HashSet<Cube>) -> Cube {
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    cubes.iter().for_each(|cube| {
        (max_x, max_y, max_z) = (max_x.max(cube.x), max_y.max(cube.y), max_z.max(cube.z));
    });
    Cube::new(max_x + 1, max_y + 1, max_z + 1)
}
