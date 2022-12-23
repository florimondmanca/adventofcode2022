use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

// Monkey in the Middle
// https://adventofcode.com/2022/day/11

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<u128> {
    let monkeys = parse(input);
    Some(simulate(&monkeys, 20, |x| x / 3))
}

fn part2(input: &str) -> Option<u128> {
    /*
    Key observation: all divisors in input are the first N prime numbers.
    We only care about 'item mod divisor' and the number of items processed.
    All integers are a product of prime numbers, so when items eventually get
    very large, the (fixed) product of the divisors will be eventually become a divisor.
    This means we can process 'item mod <product of divisors>'.
    */
    let monkeys = parse(input);
    let pod = monkeys.iter().map(|m| m.divisor).product::<u128>();
    Some(simulate(&monkeys, 10000, |x| x % pod))
}

struct Monkey<'a> {
    items: RefCell<VecDeque<u128>>,
    operation: Operation<'a>,
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

            inspected.insert(i, inspected[&i] + num_inspected as u128);

            while let Some(worry_level) = monkey.items.borrow_mut().pop_front() {
                let mut worry_level = monkey.operation.apply(worry_level);
                worry_level = relieve(worry_level);

                let target = if worry_level % monkey.divisor == 0 {
                    &monkeys[monkey.true_monkey]
                } else {
                    &monkeys[monkey.false_monkey]
                };

                target.items.borrow_mut().push_back(worry_level);
            }
        }
    }

    inspected.values().sorted().rev().take(2).product()
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("Monkey "))
        .tuples()
        .map(
            |(items_line, operation_line, divisor_line, true_monkey_line, false_monkey_line)| {
                let items = RefCell::new(
                    items_line.trim()["Starting items :".len()..]
                        .split(", ")
                        .map(|x| x.parse::<u128>().unwrap())
                        .collect(),
                );

                let operation =
                    Operation::from(&operation_line.trim()["Operation: new = ".len()..]);

                let divisor = divisor_line.trim()["Test: divisible by ".len()..]
                    .parse::<u128>()
                    .unwrap();

                let true_monkey = true_monkey_line.trim()["If true: throw to monkey ".len()..]
                    .parse::<usize>()
                    .unwrap();

                let false_monkey = false_monkey_line.trim()["If false: throw to monkey ".len()..]
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

struct Operation<'a> {
    left: &'a str,
    right: &'a str,
    op: &'a str,
}

impl<'a> Operation<'a> {
    fn new(left: &'a str, right: &'a str, op: &'a str) -> Self {
        Self { left, right, op }
    }

    fn apply(&self, value: u128) -> u128 {
        match self.op {
            "+" => resolve(self.left, value) + resolve(self.right, value),
            "*" => resolve(self.left, value) * resolve(self.right, value),
            _ => panic!("unknown operation"),
        }
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(value: &'a str) -> Self {
        let (left, op, right) = value.split(" ").collect_tuple().unwrap();
        Self::new(left, right, op)
    }
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 11);
    assert_eq!(part1(input), Some(58794));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 11);
    assert_eq!(part2(input), Some(20151213744));
}
