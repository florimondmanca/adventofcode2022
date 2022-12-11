use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/11.txt");

    let monkeys = parse(content);
    println!("Answer (part 1): {}", simulate(&monkeys, 20, |x| x / 3));

    let monkeys = parse(content);
    /*
    Key observation: all divisors in input are the first N prime numbers.
    We only care about 'item mod divisor' and the number of items processed.
    All integers are a product of prime numbers, so when items eventually get
    very large, the (fixed) product of the divisors will be eventually become a divisor.
    This means we can process 'item mod <product of divisors>'.
    */
    let pod = monkeys.iter().map(|m| m.divisor).product::<u128>();
    println!(
        "Answer (part 2): {}",
        simulate(&monkeys, 10000, |x| x % pod)
    );
}

struct Monkey {
    items: RefCell<VecDeque<u128>>,
    operation: Box<dyn Fn(u128) -> u128>,
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize,
}

fn simulate<F>(monkeys: &Vec<Monkey>, rounds: u32, relieve: F) -> u128
where
    F: Fn(u128) -> u128,
{
    let mut inspected: HashMap<usize, u128> =
        monkeys.iter().enumerate().map(|(i, _)| (i, 0)).collect();

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let num_inspected = monkey.items.borrow().len();

            inspected.insert(i, inspected.get(&i).unwrap() + num_inspected as u128);

            while let Some(worry_level) = monkey.items.borrow_mut().pop_front() {
                let mut worry_level = (monkey.operation)(worry_level);
                worry_level = relieve(worry_level);

                let target = if worry_level % monkey.divisor == 0 {
                    monkeys.get(monkey.true_monkey).unwrap()
                } else {
                    monkeys.get(monkey.false_monkey).unwrap()
                };

                target.items.borrow_mut().push_back(worry_level);
            }
        }
    }

    inspected.values().sorted().rev().take(2).product()
}

fn parse(content: &'static str) -> Vec<Monkey> {
    content
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("Monkey "))
        .tuples()
        .map(
            |(items_line, operation_line, divisor_line, true_monkey_line, false_monkey_line)| {
                let items = RefCell::new(
                    items_line
                        .trim()
                        .get("Starting items :".len()..)
                        .unwrap()
                        .split(", ")
                        .map(|x| x.parse::<u128>().unwrap())
                        .collect(),
                );

                let operation = parse_operation(
                    operation_line
                        .trim()
                        .get("Operation: new = ".len()..)
                        .unwrap(),
                );

                let divisor = divisor_line
                    .trim()
                    .get("Test: divisible by ".len()..)
                    .unwrap()
                    .parse::<u128>()
                    .unwrap();

                let true_monkey = true_monkey_line
                    .trim()
                    .get("If true: throw to monkey ".len()..)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let false_monkey = false_monkey_line
                    .trim()
                    .get("If false: throw to monkey ".len()..)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                Monkey {
                    items,
                    operation,
                    divisor,
                    true_monkey,
                    false_monkey,
                }
            },
        )
        .collect()
}

fn resolve(token: &str, old: u128) -> u128 {
    match token {
        "old" => old.clone(),
        _ => token.parse::<u128>().unwrap(),
    }
}

fn parse_operation(text: &str) -> Box<dyn Fn(u128) -> u128 + '_> {
    let (left, op, right) = text.split(" ").collect_tuple().unwrap();

    match op {
        "+" => Box::new(move |old: u128| -> u128 { resolve(left, old) + resolve(right, old) }),
        "*" => Box::new(move |old: u128| -> u128 { resolve(left, old) * resolve(right, old) }),
        _ => panic!("unknown operation"),
    }
}
