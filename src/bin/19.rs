use std::{cmp::Ordering, collections::BinaryHeap};

use regex::Regex;

fn main() {
    println!("Not Enough Minerals");
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<i32> {
    /*
     * Let Q be the 4x4 matrix representing the resource requirements
     * for building each type of robot.
     *
     * For example, this blueprint...
     *
     *  Blueprint 1:
     *      Each ore robot costs 4 ore.
     *      Each clay robot costs 2 ore.
     *      Each obsidian robot costs 3 ore and 14 clay.
     *      Each geode robot costs 2 ore and 7 obsidian.
     *
     * ...translates to:
     *
     *     ( 4  2  3  2 )
     * Q = ( 0  0  14 0 )
     *     ( 0  0  0  7 )
     *     ( 0  0  0  0 )
     *       ^ore  ^obsidian
     *          ^clay ^geode
     *
     * Each robot produces 1 unit of its resource type.
     * So, the production matrix is Y = I_4 (the 4x4 identity matrix).
     *
     * Let R_n be the 4-vector of active robots at minute n.
     * Then:
     *      R_0 = (1 0 0 0).T
     *      (Initially, we have 1 ore-collecting robot.)
     *
     * Let X_n be the 4-vector of the available resources at minute n.
     * Then:
     *      X_0 is (0 0 0 0).T
     *      (Initially, we have no spare resources.)
     *
     * Let B_n be the 4-vector describing which robot, if any, we build at minute n.
     * Each element in B_n is either 1 (build) or 0 (don't build).
     * The fact that we may only build 1 robot at a time can be translated as:
     *      ||B_n||_1 <= 1
     *      (At most 1 element is 1, with the rest being 0.)
     * The fact that we may only build a robot if we have enough resources can be translated as:
     *      Q * B_n <= X_n (element-wise)
     *
     * The iteration schema is then as follows:
     *      X_{n+1} = X_n + Y * R_n - Q * B_{n+1}
     *      R_{n+1} = R_n + B_{n+1}
     *
     * For a given blueprint, the optimal build sequence B° is the one
     * that maximizes the number of geodes after 24 minutes, i.e.:
     *      B°(Q) = argmax_{B among the set of possible build sequences} X_n_4
     *
     * The answer is the sum of ID * B° for each blueprint.
     */
    let blueprints = parse(input);

    let quality_levels = blueprints
        .into_iter()
        .map(|blueprint| blueprint.id * maximize_geodes(&blueprint, 24))
        .collect::<Vec<_>>();

    Some(quality_levels.into_iter().sum())
}

fn part2(_input: &str) -> Option<u32> {
    None
}

fn maximize_geodes(blueprint: &Blueprint, max_time: i32) -> i32 {
    let mut max_geodes = 0;
    let mut queue = BinaryHeap::new();

    let initial = State::new();
    queue.push(initial.clone());

    while let Some(state) = queue.pop() {
        for other in state.get_next_states(blueprint, max_time) {
            queue.push(other);
        }

        max_geodes = max_geodes.max(state.geodes);
    }

    max_geodes
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    time: i32,
    ore_robots: i32,
    ore: i32,
    clay_robots: i32,
    clay: i32,
    obsidian_robots: i32,
    obsidian: i32,
    geode_robots: i32,
    geodes: i32,
}

impl State {
    fn new() -> Self {
        Self {
            time: 1,
            ore_robots: 1,
            ore: 1,
            clay_robots: 0,
            clay: 0,
            obsidian_robots: 0,
            obsidian: 0,
            geode_robots: 0,
            geodes: 0,
        }
    }

    fn get_next_states(&self, blueprint: &Blueprint, max_time: i32) -> Vec<State> {
        if self.time >= max_time {
            return Vec::new();
        }

        /*
        We try building a robot if we have enough resources, and we don't have
        enough robots yet to cover the maximum amount of resources required to
        build any other kind of robot (except for geode robots: build as many of
        those as possible).
        */

        let mut next_states = Vec::new();

        if self.ore > 0 && self.ore_robots < blueprint.max_ore_cost {
            next_states.push(blueprint.ore_robot.schedule_build(&self));
        }

        if self.ore > 0 && self.clay_robots < blueprint.max_clay_cost {
            next_states.push(blueprint.clay_robot.schedule_build(&self));
        }

        if self.ore > 0 && self.clay > 0 && self.obsidian_robots < blueprint.max_obsidian_cost {
            next_states.push(blueprint.obsidian_robot.schedule_build(&self));
        }

        if self.ore > 0 && self.obsidian > 0 {
            next_states.push(blueprint.geode_robot.schedule_build(&self));
        }

        next_states
            .into_iter()
            .filter(|s| s.time <= max_time)
            .collect()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.geodes.cmp(&other.geodes)
    }
}

#[derive(Debug)]
struct RobotBlueprint {
    ore_robots_built: i32,
    clay_robots_built: i32,
    obsidian_robots_built: i32,
    geode_robots_built: i32,
    ore_cost: i32,
    clay_cost: i32,
    obsidian_cost: i32,
}

impl RobotBlueprint {
    fn new(
        ore_robots_built: i32,
        clay_robots_built: i32,
        obsidian_robots_built: i32,
        geode_robots_built: i32,
        ore_cost: i32,
        clay_cost: i32,
        obsidian_cost: i32,
    ) -> Self {
        Self {
            ore_robots_built,
            clay_robots_built,
            obsidian_robots_built,
            geode_robots_built,
            ore_cost,
            clay_cost,
            obsidian_cost,
        }
    }

    fn time_until_build(&self, state: &State) -> i32 {
        /*
        We look for the amount of time t to wait until:
            resource_cost = resource + t * resource_robots

        Rearranging, this gives:
            t = 1 + (resource_cost - resource) // resource_robots
        */

        fn comp(cost: i32, amount: i32, robots: i32) -> i32 {
            ((cost as f32 - amount as f32) / (robots as f32)).ceil() as i32
        }

        [
            if state.ore >= self.ore_cost {
                1
            } else {
                1 + comp(self.ore_cost, state.ore, state.ore_robots)
            },
            if state.clay >= self.clay_cost {
                1
            } else {
                1 + comp(self.clay_cost, state.clay, state.clay_robots)
            },
            if state.obsidian >= self.obsidian_cost {
                1
            } else {
                1 + comp(self.obsidian_cost, state.obsidian, state.obsidian_robots)
            },
        ]
        .into_iter()
        .max()
        .unwrap()
    }

    fn schedule_build(&self, state: &State) -> State {
        // How long until we can build this robot from this state?
        let time_required = self.time_until_build(&state);

        // Generate a state that will build this robot at that time,
        // and pick up new materials in the meantime.

        let mut next_state = state.clone();

        next_state.time = state.time + time_required;

        next_state.ore_robots = state.ore_robots + self.ore_robots_built;
        next_state.ore = state.ore - self.ore_cost + time_required * state.ore_robots;

        next_state.clay_robots = state.clay_robots + self.clay_robots_built;
        next_state.clay = state.clay - self.clay_cost + time_required * state.clay_robots;

        next_state.obsidian_robots = state.obsidian_robots + self.obsidian_robots_built;
        next_state.obsidian =
            state.obsidian - self.obsidian_cost + time_required * state.obsidian_robots;

        next_state.geode_robots = state.geode_robots + self.geode_robots_built;
        next_state.geodes = state.geodes + time_required * state.geode_robots;

        next_state
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot: RobotBlueprint,
    clay_robot: RobotBlueprint,
    obsidian_robot: RobotBlueprint,
    geode_robot: RobotBlueprint,
    max_ore_cost: i32,
    max_clay_cost: i32,
    max_obsidian_cost: i32,
}

impl From<&str> for Blueprint {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let cap = re.captures(line).unwrap();

        let ore_robot = RobotBlueprint::new(1, 0, 0, 0, cap[2].parse().unwrap(), 0, 0);
        let clay_robot = RobotBlueprint::new(0, 1, 0, 0, cap[3].parse().unwrap(), 0, 0);
        let obsidian_robot = RobotBlueprint::new(
            0,
            0,
            1,
            0,
            cap[4].parse().unwrap(),
            cap[5].parse().unwrap(),
            0,
        );
        let geode_robot = RobotBlueprint::new(
            0,
            0,
            0,
            1,
            cap[6].parse().unwrap(),
            0,
            cap[7].parse().unwrap(),
        );

        let max_ore_cost = [
            ore_robot.ore_cost,
            clay_robot.ore_cost,
            obsidian_robot.ore_cost,
            geode_robot.ore_cost,
        ]
        .into_iter()
        .max()
        .unwrap();

        let max_clay_cost = [
            ore_robot.clay_cost,
            clay_robot.clay_cost,
            obsidian_robot.clay_cost,
            geode_robot.clay_cost,
        ]
        .into_iter()
        .max()
        .unwrap();

        let max_obsidian_cost = [
            ore_robot.obsidian_cost,
            clay_robot.obsidian_cost,
            obsidian_robot.obsidian_cost,
            geode_robot.obsidian_cost,
        ]
        .into_iter()
        .max()
        .unwrap();

        Self {
            id: cap[1].parse().unwrap(),
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            max_ore_cost,
            max_clay_cost,
            max_obsidian_cost,
        }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from).collect()
}
