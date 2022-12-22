use std::collections::HashMap;

fn main() {
    println!("Monkey Math");
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<i64> {
    let monkeys = parse(input);
    let value = resolve_root(&monkeys);
    Some(value)
}

fn part2(_input: &str) -> Option<u32> {
    None
}

fn resolve_root(monkeys: &Vec<Monkey>) -> i64 {
    let mut values: HashMap<String, i64> = HashMap::new();

    while !values.contains_key("root") {
        for (name, job) in monkeys {
            if values.contains_key(name) {
                continue;
            }
            if let Some(value) = resolve(&job, &values) {
                values.insert(name.clone(), value);
            }
        }
    }

    *values.get("root").unwrap()
}

fn resolve(job: &Job, values: &HashMap<String, i64>) -> Option<i64> {
    match job {
        Job::Number(n) => Some(*n),
        Job::Formula(left, operator, right) => {
            let left = values.get(left)?;
            let right = values.get(right)?;
            Some(match operator {
                Operation::Add => left + right,
                Operation::Subtract => left - right,
                Operation::Multiply => left * right,
                Operation::Divide => left / right,
            })
        }
    }
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Job {
    Number(i64),
    Formula(String, Operation, String),
}

type Monkey = (String, Job);

fn parse_monkey(line: &str) -> Monkey {
    let (name, expr) = line.split_once(": ").unwrap();

    let name = name.to_string();

    if let Ok(n) = expr.parse::<i64>() {
        return (name, Job::Number(n));
    }

    let parts = expr.split_whitespace().collect::<Vec<_>>();
    let left = parts[0].to_string();
    let right = parts[2].to_string();
    let operation = match parts[1] {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => panic!(),
    };

    (name, Job::Formula(left, operation, right))
}

fn parse(input: &str) -> Vec<Monkey> {
    input.lines().map(parse_monkey).collect()
}
