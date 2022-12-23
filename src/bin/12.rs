use std::collections::{HashMap, VecDeque};

// Hill Climbing Algorithm
// https://adventofcode.com/2022/day/12

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<usize> {
    // This is a shortest path finding problem.
    // The height map can be modelled as a graph whose nodes are the
    // locations on the map, and edges are pairs of accessible locations,
    // i.e. adjacent locations whose height difference is at most 1.
    // The map is sufficiently small that we can get away with basic
    // breadth-first search (BFS) algorithm.
    let (heights, start, end) = parse(input);
    let num_steps = solve(&heights, start, end);
    Some(num_steps)
}

fn part2(input: &str) -> Option<usize> {
    // In part 2, we need to find the 'a' location which gives
    // the shortest path to E. Brute force takes a few seconds.
    let (heights, _, end) = parse(input);

    let mut shortest = usize::MAX;

    for (node, h) in heights.iter() {
        if *h == 'a' as u32 {
            let num_steps = solve(&heights, *node, end);
            if num_steps < shortest {
                shortest = num_steps;
            }
        }
    }

    Some(shortest)
}

type Node = (usize, usize);
type Heights = HashMap<Node, u32>;

fn solve(heights: &Heights, start: Node, end: Node) -> usize {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut parents: HashMap<Node, Node> = HashMap::new();
    let mut costs: HashMap<Node, u32> = HashMap::new();

    costs.insert(start, 0);
    queue.push_back(start);

    while let Some(mut node) = queue.pop_front() {
        if node == end {
            let mut path = Vec::new();
            while let Some(parent) = parents.get(&node) {
                let parent = *parent;
                path.push(parent);
                node = parent;
            }
            return path.len();
        }

        let h = heights[&node];

        for neighbor in find_neighbors(heights, node) {
            let neighbor_h = heights[&neighbor];

            if neighbor_h > h + 1 {
                continue;
            }

            let tentative_cost = costs[&node] + 1;

            if tentative_cost < *costs.get(&neighbor).unwrap_or(&u32::MAX) {
                costs.insert(neighbor, tentative_cost);
                parents.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }

    usize::MAX
}

fn find_neighbors(heights: &Heights, node: Node) -> Vec<Node> {
    let mut neighbors = vec![];
    let (row, col) = node;

    if col >= 1 {
        if heights.contains_key(&(row, col - 1)) {
            neighbors.push((row, col - 1));
        }
    }

    if heights.contains_key(&(row, col + 1)) {
        neighbors.push((row, col + 1));
    }

    if row >= 1 {
        if heights.contains_key(&(row - 1, col)) {
            neighbors.push((row - 1, col));
        }
    }

    if heights.contains_key(&(row + 1, col)) {
        neighbors.push((row + 1, col));
    }

    neighbors
}

fn parse(content: &str) -> (Heights, Node, Node) {
    let mut heights: Heights = HashMap::new();
    let mut start: Option<Node> = None;
    let mut end: Option<Node> = None;

    for (row, line) in content.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let node = (row, col);

            if c == 'S' {
                assert!(start.is_none());
                start = Some(node);
                heights.insert(node, 'a' as u32 - 1);
                continue;
            }

            if c == 'E' {
                assert!(end.is_none());
                end = Some(node);
                heights.insert(node, 'z' as u32 + 1);
                continue;
            }

            heights.insert(node, c as u32);
        }
    }

    (heights, start.unwrap(), end.unwrap())
}

#[test]
#[ignore]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 12);
    assert_eq!(part1(input), Some(391));
}

#[test]
#[ignore]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 12);
    assert_eq!(part2(input), Some(386));
}
