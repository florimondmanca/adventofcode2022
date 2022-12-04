use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/4.txt");
    part1(content);
}

fn part1(content: &str) {
    let num_overlap = content
        .lines()
        .map(read_sorted_ranges)
        .map(|(smallest, longest)| longest.contains(smallest) as u32)
        .sum::<u32>();

    println!("Answer (part 1): {}", num_overlap);
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Range { start, end }
    }

    fn length(&self) -> u32 {
        self.end - self.start
    }

    fn contains(&self, other: Range) -> bool {
        return self.start <= other.start && other.end <= self.end;
    }
}

fn read_sorted_ranges(line: &str) -> (Range, Range) {
    line.split(',')
        .map(read_range)
        .take(2)
        .sorted_by(|a, b| a.length().cmp(&b.length()))
        .collect_tuple()
        .unwrap()
}

fn read_range(rng: &str) -> Range {
    rng.split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .take(2)
        .tuples()
        .map(|(start, end)| Range::new(start, end))
        .nth(0)
        .unwrap()
}
