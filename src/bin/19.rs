use std::io::{stdout, Write};

use itertools::Itertools;
use regex::Regex;

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
     * Let X_n = (x_n, y_n, z_n, w_n) be the 4-vector of the available resources at minute n.
     * Then:
     *      X_0 is (0 0 0 0).T
     *      (Initially, we have no spare resources.)
     *
     * Finally, let B_n = (a_n, b_n, c_n, d_n) be the number of robots
     * of each type we decide to build at minute n.
     * Then:
     *      0 <= B_n < X_n
     *
     * Then:
     *      R_{n+1} = R_n + B_n
     *      X_{n+1} = X_n + Y * R_n - Q * B_n
     *
     * The optimal sequence of robot manufacturing for a blueprint defined by the
     * requirements matrix Q is the one that maximizes the number of geodes at minute 24, i.e.:
     *      B°(Q) = argmax_{B = [B_0, B_1, ..., B_24]} w_24
     *
     * The answer is the sum of ID * B° for each blueprint.
     */
    println!();

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
    // When should we build robots? (This is a backtracking problem.)
    // At each step, there is a set of buildable robots, as defined by
    // those for which we have enough resources.
    // Assume this set is of size N (N may be 0 up to 4).
    // Then at each step we have N + 1 choices: build one of the N robots, or build none.
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

fn simulate(blueprint: &Blueprint, build_seq: Vec<[u32; 4]>) -> u32 {
    println!("+--------------+");
    println!("| Blueprint {:02} |", blueprint.id);
    println!("+--------------+");
    println!();

    let resource_types = ["ore", "clay", "obsidian", "geode"];

    let mut steps = vec![([0, 0, 0, 0], [1, 0, 0, 0])];

    for (index, &build) in build_seq.iter().enumerate() {
        let minute = index + 1;

        // Lock stdout for the entire iteration, rather locking it
        // each print!() during this hot loop.
        let mut out = stdout().lock();

        writeln!(out, "== Minute {} ==", minute).unwrap();
        let (resources_nprev, robots_nprev) = steps[minute - 1];

        let mut resources_n = resources_nprev.clone();
        let mut robots_n = robots_nprev.clone();
        let mut in_the_making = [0, 0, 0, 0];

        for (dim, &resource_type) in resource_types.iter().enumerate() {
            match build[dim] {
                0 => {}
                1 => {
                    let req = blueprint.requirements[dim];

                    writeln!(
                        out,
                        "Spend {spending} to start building a {resource}-collecting robot.",
                        spending = req
                            .clone()
                            .iter()
                            .zip(resource_types)
                            .filter(|(&amount, _)| amount > 0)
                            .map(|(amount, resource)| format!("{amount} {resource}"))
                            .join(", "),
                        resource = resource_type
                    )
                    .unwrap();

                    for (j, amount) in req.iter().enumerate() {
                        resources_n[j] -= amount;
                    }

                    in_the_making[dim] += 1;
                }
                _ => panic!("Illegal number of robots to build: {}", build[dim]),
            }
        }

        for (dim, &resource_type) in resource_types.iter().enumerate() {
            if robots_nprev[dim] > 0 {
                write!(
                    out,
                    "{num} {resource}-{actioning} {robots} {action} {num} {resources}; ",
                    num = robots_nprev[dim],
                    resource = resource_type,
                    actioning = match resource_type {
                        "geode" => "breaking",
                        _ => "collecting",
                    },
                    robots = pluralize(robots_nprev[dim], "robot", "robots"),
                    action = match resource_type {
                        "geode" => pluralize(robots_nprev[dim], "cracks", "crack"),
                        _ => pluralize(robots_nprev[dim], "collects", "collect"),
                    },
                    resources = match resource_type {
                        "geode" => "geodes",
                        t => t,
                    },
                )
                .unwrap();

                resources_n[dim] += robots_nprev[dim];

                writeln!(
                    out,
                    "you now have {num} {resource}.",
                    num = resources_n[dim],
                    resource = match resource_type {
                        "geode" => pluralize(resources_n[dim], "open geode", "open geodes"),
                        t => t,
                    }
                )
                .unwrap();
            }

            if in_the_making[dim] > 0 {
                robots_n[dim] += in_the_making[dim];
                writeln!(
                    out,
                    "The new {resource}-collecting robot is ready; you now have {num} of them.",
                    resource = resource_types[dim],
                    num = robots_n[dim]
                )
                .unwrap();
            }
        }

        println!();

        steps.push((resources_n, robots_n));
    }

    let (resources_final, _) = steps.last().unwrap();

    resources_final[3]
}

type RequirementsMatrix = [[u32; 4]; 4];

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

            let requirements = [
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
