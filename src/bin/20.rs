// Grove Positioning System
// https://adventofcode.com/2022/day/20

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<i64> {
    let mut numbers = parse(input, 1);
    Some(decrypt(&mut numbers, 1))
}

fn part2(input: &str) -> Option<i64> {
    let mut numbers = parse(input, 811589153);
    Some(decrypt(&mut numbers, 10))
}

struct Number {
    value: i64,
    original_position: usize,
}

fn parse<'a>(input: &'a str, decryption_key: i64) -> Vec<Number> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, n)| Number {
            value: n * decryption_key,
            original_position: i,
        })
        .collect()
}

fn decrypt(numbers: &mut Vec<Number>, num_mixes: usize) -> i64 {
    let size = numbers.len() as i64 - 1;

    for _ in 0..num_mixes {
        for current in 0..numbers.len() {
            let index = numbers
                .iter()
                .position(|x| x.original_position == current)
                .unwrap();

            let mut new_index = index as i64 + numbers[index].value;
            new_index = ((new_index % size) + size) % size;

            let number = numbers.remove(index);
            numbers.insert(new_index as usize, number);
        }
    }

    let zero_ix = numbers.iter().position(|x| x.value == 0).unwrap();
    let x = numbers[(zero_ix + 1000) % numbers.len()].value;
    let y = numbers[(zero_ix + 2000) % numbers.len()].value;
    let z = numbers[(zero_ix + 3000) % numbers.len()].value;

    x + y + z
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 20);
    assert_eq!(part1(input), Some(8302));
}

#[test]
#[ignore = "slow"]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 20);
    assert_eq!(part2(input), Some(656575624777));
}
