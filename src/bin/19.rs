use std::io::{stdout, StdoutLock, Write};

use itertools::Itertools;
use regex::Regex;

const RESOURCE_TYPES: [&str; 4] = ["ore", "clay", "obsidian", "geode"];

fn main() {
    println!("Not Enough Minerals");
    let input = &advent_of_code::read_file("examples", 19);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<u32> {
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

    let mut quality_sum = 0;

    for blueprint in blueprints {
        let max_num_geodes = maximize_geodes(&blueprint);
        quality_sum += blueprint.id * max_num_geodes;
    }

    Some(quality_sum)
}

fn part2(_input: &str) -> Option<u32> {
    None
}

fn maximize_geodes(blueprint: &Blueprint) -> u32 {
    /*
    When should we build robots? This is a backtracking problem.

    At each step, the amount of available resources defines the set of buildable robot types.
    Assume this set is of size N, which can be 0, 1, 2, 3 or 4.

    Then at each step we must decide among N + 1 possibilities:
    * Build one of the N robots (N branches);
    * Build none.

    For a number of minutes M, a brute force approach would then have
    a complexity of O(4^M). This is of course ridiculous and not even worth trying.
    (24 minutes may be manageable, but part 2 will probably want us to solve this problem
    for a much larger number of minutes. So we must be smarter than brute force.)
     */
    let mut build_seq = Vec::new();

    for minute in 1..=24 {
        if blueprint.id == 1 {
            build_seq.push(match minute {
                3 => [0, 1, 0, 0],
                5 => [0, 1, 0, 0],
                7 => [0, 1, 0, 0],
                11 => [0, 0, 1, 0],
                12 => [0, 1, 0, 0],
                15 => [0, 0, 1, 0],
                18 => [0, 0, 0, 1],
                21 => [0, 0, 0, 1],
                _ => [0, 0, 0, 0],
            });
        } else {
            build_seq.push([0, 0, 0, 0]);
        }
    }

    simulate(blueprint, build_seq)
}

type Vec4 = [u32; 4];
type Build = Vec4;
type Step = (Vec4, Vec4);

fn simulate(blueprint: &Blueprint, build_seq: Vec<Build>) -> u32 {
    Simulation::new(blueprint, build_seq).run()
}

struct Simulation<'a> {
    blueprint: &'a Blueprint,
    build_seq: Vec<Build>,
    steps: Vec<Step>,
}

impl<'a> Simulation<'a> {
    fn new(blueprint: &'a Blueprint, build_seq: Vec<Build>) -> Self {
        let steps = vec![
            // Initially...
            (
                [0, 0, 0, 0], // No resources
                [1, 0, 0, 0], // 1 ore-collecting robot
            ),
        ];

        Self {
            blueprint,
            build_seq,
            steps,
        }
    }

    fn maybe_start_building_a_robot(
        &self,
        out: &mut StdoutLock,
        build: Build,
        resources_next: &mut Vec4,
        robots_in_the_making: &mut Vec4,
    ) {
        RESOURCE_TYPES
            .iter()
            .enumerate()
            .filter(|(dim, _)| build[*dim] == 1)
            .for_each(|(dim, &resource_type)| {
                let robot_requirements = self.blueprint.requirements[dim];

                writeln!(
                    out,
                    "Spend {spending} to start building a {resource}-collecting robot.",
                    spending = robot_requirements
                        .clone()
                        .iter()
                        .zip(RESOURCE_TYPES)
                        .filter(|(&amount, _)| amount > 0)
                        .map(|(amount, resource)| format!("{amount} {resource}"))
                        .join(", "),
                    resource = resource_type
                )
                .unwrap();

                for (i, amount) in robot_requirements.iter().enumerate() {
                    resources_next[i] -= amount;
                }

                robots_in_the_making[dim] += 1;
            });
    }

    fn collect_resources_with_existing_robots(
        &self,
        out: &mut StdoutLock,
        robots_prev: Vec4,
        resources_next: &mut Vec4,
    ) {
        RESOURCE_TYPES
            .iter()
            .enumerate()
            .filter(|(dim, _)| robots_prev[*dim] > 0)
            .for_each(|(dim, &resource_type)| {
                write!(
                    out,
                    "{num} {resource}-{actioning} {robots} {action} {num} {resources}; ",
                    num = robots_prev[dim],
                    resource = resource_type,
                    actioning = match resource_type {
                        "geode" => "breaking",
                        _ => "collecting",
                    },
                    robots = pluralize(robots_prev[dim], "robot", "robots"),
                    action = match resource_type {
                        "geode" => pluralize(robots_prev[dim], "cracks", "crack"),
                        _ => pluralize(robots_prev[dim], "collects", "collect"),
                    },
                    resources = match resource_type {
                        "geode" => "geodes",
                        t => t,
                    },
                )
                .unwrap();

                resources_next[dim] += robots_prev[dim];

                writeln!(
                    out,
                    "you now have {num} {resource}.",
                    num = resources_next[dim],
                    resource = match resource_type {
                        "geode" => pluralize(resources_next[dim], "open geode", "open geodes"),
                        t => t,
                    }
                )
                .unwrap();
            });
    }

    fn finish_building_any_new_robot(
        &self,
        out: &mut StdoutLock,
        robots_in_the_making: Vec4,
        robots_next: &mut Vec4,
    ) {
        RESOURCE_TYPES
            .iter()
            .enumerate()
            .filter(|(dim, _)| robots_in_the_making[*dim] > 0)
            .for_each(|(dim, &resource_type)| {
                robots_next[dim] += robots_in_the_making[dim];
                writeln!(
                    out,
                    "The new {resource}-collecting robot is ready; you now have {num} of them.",
                    resource = resource_type,
                    num = robots_next[dim]
                )
                .unwrap();
            });
    }

    fn advance(&self, out: &mut StdoutLock, minute: usize, build: Build) -> Step {
        writeln!(out, "== Minute {} ==", minute).unwrap();
        let (resources_prev, robots_prev) = self.steps[minute - 1];

        let mut resources_next = resources_prev.clone();
        let mut robots_next = robots_prev.clone();
        let mut robots_in_the_making = [0, 0, 0, 0];

        self.maybe_start_building_a_robot(
            out,
            build,
            &mut resources_next,
            &mut robots_in_the_making,
        );
        self.collect_resources_with_existing_robots(out, robots_prev, &mut resources_next);
        self.finish_building_any_new_robot(out, robots_in_the_making, &mut robots_next);

        writeln!(out).unwrap();

        (resources_next, robots_next)
    }

    fn run(&mut self) -> u32 {
        println!();
        println!("+--------------+");
        println!("| Blueprint {:02} |", self.blueprint.id);
        println!("+--------------+");
        println!();

        for (index, &build) in self.build_seq.iter().enumerate() {
            // Lock stdout for the entire iteration, rather locking it
            // each print!() during this hot loop.
            let mut out = stdout().lock();
            let minute = index + 1;
            let step = self.advance(&mut out, minute, build);
            self.steps.push(step);
        }

        let (resources_final, _) = self.steps.last().unwrap();

        resources_final[3]
    }
}

type RequirementsMatrix = [Vec4; 4];

struct Blueprint {
    id: u32,
    requirements: RequirementsMatrix,
}

impl Blueprint {
    fn new(id: u32, requirements: RequirementsMatrix) -> Self {
        Self { id, requirements }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let id = cap[1].parse().unwrap();

            let requirements: RequirementsMatrix = [
                [cap[2].parse().unwrap(), 0, 0, 0],
                [cap[3].parse().unwrap(), 0, 0, 0],
                [cap[4].parse().unwrap(), cap[5].parse().unwrap(), 0, 0],
                [cap[6].parse().unwrap(), 0, cap[7].parse().unwrap(), 0],
            ];

            Blueprint::new(id, requirements)
        })
        .collect()
}

fn pluralize<'a>(amount: u32, one: &'a str, zero_or_many: &'a str) -> &'a str {
    if amount == 1 {
        one
    } else {
        zero_or_many
    }
}
