use std::{cmp::Ordering, collections::BinaryHeap, ops::RangeInclusive};

use regex::Regex;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const RESOURCES: RangeInclusive<usize> = ORE..=GEODE;

fn main() {
    println!("Not Enough Minerals");
    // Kudos to: https://todd.ginsberg.com/post/advent-of-code/2022/day19/
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::slow!(|| {
        advent_of_code::solve!(1, part1, input);
        advent_of_code::solve!(2, part2, input);
    });
}

fn part1(input: &str) -> Option<i32> {
    let blueprints = parse(input);

    let quality_levels = blueprints
        .into_iter()
        .map(|blueprint| blueprint.id * maximize_geodes(&blueprint, 24))
        .collect::<Vec<_>>();

    Some(quality_levels.into_iter().sum())
}

fn part2(input: &str) -> Option<i32> {
    let blueprints = parse(input);

    let max_geodes = blueprints
        .into_iter()
        .take(3)
        .map(|blueprint| maximize_geodes(&blueprint, 32))
        .collect::<Vec<_>>();

    Some(max_geodes.into_iter().product())
}

fn maximize_geodes(blueprint: &Blueprint, max_time: i32) -> i32 {
    // Perform a depth-first search (DFS) on the set of possible states,
    // using the number of geodes as the heuristic,
    // skipping states for which we cannot ever beat the best amount we found so far.
    let mut max_geodes = 0;
    let mut queue = BinaryHeap::new();

    let initial = State::new(1, [1, 0, 0, 0], [1, 0, 0, 0]);
    queue.push(initial.clone());

    while let Some(state) = queue.pop() {
        if !state.can_beat(max_geodes, max_time) {
            continue;
        }

        for other in state.get_next_states(blueprint, max_time) {
            queue.push(other);
        }

        max_geodes = max_geodes.max(state.resources[GEODE]);
    }

    max_geodes
}

type Vec4 = [i32; 4];

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    time: i32,
    robots: Vec4,
    resources: Vec4,
}

impl State {
    fn new(time: i32, robots: Vec4, resources: Vec4) -> Self {
        Self {
            time,
            robots,
            resources,
        }
    }

    fn get_next_states(&self, blueprint: &Blueprint, max_time: i32) -> Vec<State> {
        if self.time >= max_time {
            return Vec::new();
        }

        let mut next_states = Vec::new();

        for res in RESOURCES {
            let requirements = match res {
                ORE => vec![self.resources[ORE]],
                CLAY => vec![self.resources[ORE]],
                OBSIDIAN => vec![self.resources[ORE], self.resources[CLAY]],
                GEODE => vec![self.resources[ORE], self.resources[OBSIDIAN]],
                _ => panic!(),
            };

            let has_requirements = requirements.into_iter().all(|amount| amount > 0);

            let max_cost = blueprint.max_costs[res];
            let want_more_of_this_robot = max_cost == 0 || self.robots[res] < max_cost;

            if has_requirements && want_more_of_this_robot {
                next_states.push(blueprint.robots[res].schedule_build(&self));
            }
        }

        next_states
            .into_iter()
            .filter(|s| s.time <= max_time)
            .collect()
    }

    fn can_beat(&self, best_so_far: i32, max_time: i32) -> bool {
        let time_left = max_time - self.time;
        let potential_geodes = (0..time_left).map(|n| n + self.robots[GEODE]).sum::<i32>();
        self.resources[GEODE] + potential_geodes > best_so_far
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.resources[GEODE].cmp(&other.resources[GEODE])
    }
}

#[derive(Debug)]
struct RobotBlueprint {
    robots_built: Vec4,
    costs: Vec4,
}

impl RobotBlueprint {
    fn new(robots_built: Vec4, costs: Vec4) -> Self {
        Self {
            robots_built,
            costs,
        }
    }

    fn time_until_build(&self, state: &State) -> i32 {
        /*
        We look for the amount of time t to wait until:
            resource_cost = resource + t * resource_robots

        Rearranging, this gives:
            t = 1 + (resource_cost - resource) // resource_robots
        */

        let mut time = 0;

        for res in RESOURCES {
            if state.resources[res] >= self.costs[res] {
                time = time.max(1);
                continue;
            }

            let num_to_collect = self.costs[res] as f32 - state.resources[res] as f32;
            let num_robots = state.robots[res] as f32;
            let num_steps = (num_to_collect / num_robots).ceil() as i32;
            time = time.max(1 + num_steps);
        }

        time
    }

    fn schedule_build(&self, state: &State) -> State {
        // How long until we can build this robot from this state?
        let time_required = self.time_until_build(&state);

        // Generate a state that will build this robot at that time,
        // and pick up new materials in the meantime.

        let time = state.time + time_required;
        let mut robots = [0, 0, 0, 0];
        let mut resources = [0, 0, 0, 0];

        for res in RESOURCES {
            robots[res] = state.robots[res] + self.robots_built[res];
            resources[res] =
                state.resources[res] - self.costs[res] + time_required * state.robots[res];
        }

        State::new(time, robots, resources)
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    robots: [RobotBlueprint; 4],
    max_costs: Vec4,
}

impl From<&str> for Blueprint {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let cap = re.captures(line).unwrap();

        let id = cap[1].parse().unwrap();

        let robots = [
            RobotBlueprint::new([1, 0, 0, 0], [cap[2].parse().unwrap(), 0, 0, 0]),
            RobotBlueprint::new([0, 1, 0, 0], [cap[3].parse().unwrap(), 0, 0, 0]),
            RobotBlueprint::new(
                [0, 0, 1, 0],
                [cap[4].parse().unwrap(), cap[5].parse().unwrap(), 0, 0],
            ),
            RobotBlueprint::new(
                [0, 0, 0, 1],
                [cap[6].parse().unwrap(), 0, cap[7].parse().unwrap(), 0],
            ),
        ];

        let max_costs = [
            robots.iter().map(|r| r.costs[ORE]).max().unwrap(),
            robots.iter().map(|r| r.costs[CLAY]).max().unwrap(),
            robots.iter().map(|r| r.costs[OBSIDIAN]).max().unwrap(),
            robots.iter().map(|r| r.costs[GEODE]).max().unwrap(),
        ];

        Self {
            id,
            robots,
            max_costs,
        }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from).collect()
}
