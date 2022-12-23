/*
*/
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

// Proboscidea Volcanium
// https://adventofcode.com/2022/day/16

// Kudos: https://gitlab.com/landreville/advent-of-code-2022/-/blob/master/src/day16.rs

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<i32> {
    let mut valves = Vec::new();
    let network = parse(input, &mut valves);
    let reach_times = get_reach_times(&network);

    let start = *network.keys().find(|&v| v.name == "AA").unwrap();

    let available_valves = network
        .keys()
        .filter(|v| v.flow_rate > 0)
        .map(|v| *v)
        .collect::<Vec<&Valve>>();

    let path = find_path(
        &network,
        &available_valves,
        &reach_times,
        start,
        30,
        &vec![start],
    );

    Some(path.released_pressure)
}

fn part2(_input: &str) -> Option<i32> {
    None
}

struct Path<'a> {
    valves: Vec<&'a Valve>,
    released_pressure: i32,
}

fn find_path<'a>(
    network: &'a Network,
    available_valves: &Vec<&'a Valve>,
    reach_times: &'a ReachTimes,
    start: &Valve,
    time_left: i32,
    path: &Vec<&'a Valve>,
) -> Path<'a> {
    let mut paths: Vec<Path> = Vec::new();

    for valve in available_valves {
        let reach_time = reach_times[&start.name][&valve.name];
        if reach_time >= time_left {
            continue;
        }

        let time_left_after_opening_valve = time_left - reach_time - 1;
        let pressure_released_by_valve = valve.flow_rate * time_left_after_opening_valve;

        let next_available_valves = available_valves
            .iter()
            .filter(|v| *v != valve)
            .map(|v| *v)
            .collect();

        // Recursively find the path with maximum pressure release,
        // starting from this valve.

        let mut next_path = path.clone();
        next_path.push(valve);

        let full_path = find_path(
            network,
            &next_available_valves,
            reach_times,
            valve,
            time_left_after_opening_valve,
            &next_path,
        );

        let mut extended_path = path.clone();
        extended_path.extend(full_path.valves);

        paths.push(Path {
            valves: extended_path,
            released_pressure: full_path.released_pressure + pressure_released_by_valve,
        });
    }

    let mut best_path = Path {
        valves: Vec::new(),
        released_pressure: 0,
    };

    for path in paths {
        if path.released_pressure > best_path.released_pressure {
            best_path = path;
        }
    }

    best_path
}

#[derive(PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: i32,
}

// The valve network graph, represented using adjacency lists.
// {AA -> [BB, CC, ...], ...}
type Network<'a> = HashMap<&'a Valve, Vec<&'a Valve>>;

// {AA -> {BB -> 13, CC -> 54, ...}, ...}
type ReachTimes<'a> = HashMap<String, HashMap<String, i32>>;

fn parse<'a>(content: &str, valves: &'a mut Vec<Valve>) -> Network<'a> {
    let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();

    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
        .unwrap();

    for line in content.lines() {
        let cap = re.captures(line).unwrap();

        let name = cap[1].to_string();
        let flow_rate = cap[2].parse::<i32>().unwrap();

        valves.push(Valve {
            name: name.clone(),
            flow_rate,
        });
        neighbors.insert(name, cap[3].split(", ").map(|s| s.to_string()).collect());
    }

    let mut network: Network = HashMap::new();

    for valve in valves.iter() {
        for neighbor in neighbors[&valve.name].iter() {
            let neighbor_valve = valves.iter().find(|v| &v.name == neighbor).unwrap();
            network
                .entry(valve)
                .or_insert(Vec::new())
                .push(neighbor_valve);
        }
    }

    network
}

struct Visit<'a> {
    valve: &'a Valve,
    total_time: i32,
}

impl<'a> Ord for Visit<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // v1 <= v2 (has lower priority) <=> v2 takes less time than v1
        other.total_time.cmp(&self.total_time)
    }
}

impl<'a> PartialOrd for Visit<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a> PartialEq for Visit<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.total_time.eq(&other.total_time)
    }
}

impl<'a> Eq for Visit<'a> {}

fn get_reach_times<'a>(network: &'a Network) -> ReachTimes<'a> {
    let mut reach_times = HashMap::new();

    for start in network.keys() {
        let start = *start;

        // Compute the minimum time to reach every other valve
        // in the network if beginning from this start valve.
        let times_from = reach_times
            .entry(start.name.clone())
            .or_insert(HashMap::new());

        // A BinaryHeap keeps items in sorted order, with highest priority first.
        // Here, priority is the smallest known total time.
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();

        to_visit.push(Visit {
            valve: start,
            total_time: 0,
        });

        while let Some(Visit { valve, total_time }) = to_visit.pop() {
            if !visited.insert(valve) {
                continue;
            }

            for &neighbor in network[valve].iter() {
                // Moving from one valve to another takes 1 minute.
                let new_total_time = total_time + 1;

                // Is going through this `valve` a shorter way to get from `start` to `neighbor`?
                let is_shorter = times_from
                    .get(&neighbor.name)
                    .map_or(true, |&current_total_time| {
                        current_total_time > new_total_time
                    });

                if is_shorter {
                    times_from.insert(neighbor.name.clone(), new_total_time);

                    to_visit.push(Visit {
                        valve: neighbor,
                        total_time: new_total_time,
                    });
                }
            }
        }
    }

    reach_times
}

#[test]
#[ignore]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 16);
    assert_eq!(part1(input), Some(1789));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 16);
    assert_eq!(part2(input), None);
}
