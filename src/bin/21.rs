use std::collections::HashMap;

fn main() {
    println!("Monkey Math");
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<i64> {
    let monkeys = parse(input);
    let value = resolve_until_has_value("root", &monkeys, &mut HashMap::new());
    Some(value)
}

fn part2(input: &str) -> Option<i64> {
    let monkeys = parse(input);

    let mut values: HashMap<String, i64> = HashMap::new();

    let monkeys = {
        // The strategy is to forcibly resolve root to 0, then
        // introduce all the inverse operations for existing formula monkeys.
        // When solving for humn,

        let (_, root) = monkeys.iter().find(|(name, _)| name == "root").unwrap();

        let mut extended_monkeys = monkeys
            .iter()
            .filter(|(name, _)| name != "humn" && name != "root")
            .map(|monkey| monkey.clone())
            .collect::<Vec<Monkey>>();

        values.insert(String::from("root"), 0);
        extended_monkeys.push((
            "root".to_string(),
            match root {
                Job::Number(_) => panic!("root must be a formula"),
                Job::Formula(left, _, right) => {
                    Job::Formula(right.clone(), Operation::Subtract, left.clone())
                }
            },
        ));

        extended_monkeys.extend(
            extended_monkeys
                .iter()
                .flat_map(|m| build_inverse(m))
                .collect::<Vec<_>>(),
        );

        extended_monkeys
    };

    let value = resolve_until_has_value("humn", &monkeys, &mut values);
    Some(value)
}

#[derive(Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
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

fn resolve_until_has_value(
    node: &str,
    monkeys: &Vec<Monkey>,
    values: &mut HashMap<String, i64>,
) -> i64 {
    while !values.contains_key(node) {
        for (name, job) in monkeys.iter() {
            if values.contains_key(name) {
                continue;
            }
            if let Some(value) = resolve(&job, &values) {
                values.insert(name.clone(), value);
            }
        }
    }

    *values.get(node).unwrap()
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

fn build_inverse(monkey: &Monkey) -> Vec<Monkey> {
    use Job::*;
    use Operation::*;

    let (name, job) = monkey;

    match job {
        Number(_) => vec![],
        // if   name: left + right
        // then left: name - right
        // and  right: name - left
        Formula(left, Add, right) => {
            vec![
                (left.clone(), Formula(name.clone(), Subtract, right.clone())),
                (right.clone(), Formula(name.clone(), Subtract, left.clone())),
            ]
        }
        // Etc...
        Formula(left, Subtract, right) => {
            vec![
                (left.clone(), Formula(name.clone(), Add, right.clone())),
                (right.clone(), Formula(left.clone(), Subtract, name.clone())),
            ]
        }
        Formula(left, Multiply, right) => {
            vec![
                (left.clone(), Formula(name.clone(), Divide, right.clone())),
                (right.clone(), Formula(name.clone(), Divide, left.clone())),
            ]
        }
        Formula(left, Divide, right) => {
            vec![
                (left.clone(), Formula(name.clone(), Multiply, right.clone())),
                (right.clone(), Formula(left.clone(), Divide, name.clone())),
            ]
        }
    }
}
