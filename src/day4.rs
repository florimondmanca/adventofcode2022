use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/4.txt");

    let num_full_overlaps = content
        .lines()
        .map(read_ranges)
        .map(|(a, b)| -> (Range, Range) {
            if a.length() <= b.length() {
                (a, b)
            } else {
                (b, a)
            }
        })
        .map(|(smallest, longest)| {
            (longest.start <= smallest.start && smallest.end <= longest.end) as u32
        })
        .sum::<u32>();

    println!("Answer (part 1): {num_full_overlaps}");

    let num_overlaps = content
        .lines()
        .map(read_ranges)
        .map(|(a, b)| -> (Range, Range) {
            if a.start <= b.start {
                (a, b)
            } else {
                (b, a)
            }
        })
        .map(|(leftmost, rightmost)| (rightmost.start <= leftmost.end) as u32)
        .sum::<u32>();

    println!("Answer (part 2): {num_overlaps}");
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
}

fn read_ranges(line: &str) -> (Range, Range) {
    // 1-3,5-8 -> ((1, 3), (5, 8))
    line.split(',')
        .map(|rng| {
            rng.split('-')
                .map(|section| section.parse::<u32>().unwrap())
                .tuples()
                .map(|(start, end)| Range::new(start, end))
                .nth(0)
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}
