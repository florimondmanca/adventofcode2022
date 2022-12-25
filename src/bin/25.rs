// Day 25
// https://adventofcode.com/2022/day/25

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<Snafu> {
    let snafu = input
        .lines()
        .map(|line| Decimal::from(Snafu::from(line).clone()))
        .sum::<Decimal>()
        .into();

    Some(snafu)
}

fn part2(_input: &str) -> Option<u32> {
    None
}

#[derive(Clone, PartialEq, Eq)]
struct Snafu(Vec<char>);

impl Snafu {
    fn items(&'_ self) -> Box<dyn Iterator<Item = (u32, char)> + '_> {
        let highest_exponent = self.0.len() as u32;
        Box::new((0..=highest_exponent).zip(self.0.iter().rev().map(|&c| c)))
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string()).unwrap();
        Ok(())
    }
}

impl std::fmt::Debug for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string()).unwrap();
        Ok(())
    }
}

impl From<&str> for Snafu {
    fn from(s: &str) -> Self {
        Self(s.chars().collect())
    }
}

type Decimal = u64;

impl From<Decimal> for Snafu {
    fn from(n: Decimal) -> Self {
        let mut digits = Vec::new();
        let mut n = n.clone();
        let mut carryover = 0;

        loop {
            let mut k = n % 5;
            n /= 5;

            k += carryover;
            carryover = 0;

            let c = match k {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => {
                    carryover += 1;
                    '='
                }
                4 => {
                    carryover += 1;
                    '-'
                }
                5 => {
                    carryover += 1;
                    '0'
                }
                6 => {
                    carryover += 1;
                    '1'
                }
                _ => unreachable!(),
            };

            digits.push(c);

            if n == 0 {
                match carryover {
                    0 => {}
                    1 => digits.push('1'),
                    _ => unreachable!(),
                };

                break;
            }
        }

        let chars = digits.into_iter().rev().collect::<Vec<char>>();

        Self(chars)
    }
}

impl From<Snafu> for Decimal {
    fn from(s: Snafu) -> Self {
        let mut d = 0;

        for (exponent, c) in s.items() {
            let k = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };

            d += k * 5_i64.pow(exponent);
        }

        d as Self
    }
}

#[test]
fn test_examples() {
    assert_eq!(Snafu::from(1), Snafu::from("1"));
    assert_eq!(Snafu::from(2), Snafu::from("2"));
    assert_eq!(Snafu::from(3), Snafu::from("1="));
    assert_eq!(Snafu::from(4), Snafu::from("1-"));
    assert_eq!(Snafu::from(5), Snafu::from("10"));
    assert_eq!(Snafu::from(6), Snafu::from("11"));
    assert_eq!(Snafu::from(7), Snafu::from("12"));
    assert_eq!(Snafu::from(8), Snafu::from("2="));
    assert_eq!(Snafu::from(9), Snafu::from("2-"));
    assert_eq!(Snafu::from(10), Snafu::from("20"));
    assert_eq!(Snafu::from(15), Snafu::from("1=0"));
    assert_eq!(Snafu::from(20), Snafu::from("1-0"));
    assert_eq!(Snafu::from(2022), Snafu::from("1=11-2"));
    assert_eq!(Snafu::from(12345), Snafu::from("1-0---0"));
    assert_eq!(Snafu::from(314159265), Snafu::from("1121-1110-1=0"));
}

#[test]
fn test_25_part1() {
    let input = &advent_of_code::read_file("inputs", 25);
    assert_eq!(part1(input), Some(Snafu::from("122-2=200-0111--=200")));
}

#[test]
fn test_25_part2() {
    let input = &advent_of_code::read_file("inputs", 25);
    assert_eq!(part2(input), None);
}
