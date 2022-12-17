use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

pub fn run() {
    let example = include_str!("inputs/15.example.txt");
    let sensors = parse(example);
    println!("Example (Part 1): {}", count_beacon_forbidden(sensors, 10));

    let content = include_str!("inputs/15.txt");
    let sensors = parse(content);
    println!("Answer (Part 1): {}", count_beacon_forbidden(sensors, 2000000));
}

fn count_beacon_forbidden(sensors: Vec<Sensor>, y: i32) -> usize {
    let views = sensors.iter().map(|s| s.get_view(y)).collect();

    reduce(views)
        .iter()
        .map(|range| (range.end - range.start) as usize)
        .sum::<usize>() - 1 // 1 beacon must be on this row
}

struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Sensor {
    loc: Point2D,
    radius: i32,
}

impl Sensor {
    fn new(loc: Point2D, closest_beacon: Point2D) -> Self {
        let radius = (closest_beacon.x - loc.x).abs() + (closest_beacon.y - loc.y).abs();
        Self { loc, radius }
    }

    fn get_view(&self, y: i32) -> Range<i32> {
        /*
        Return the range of columns that this sensor can view when on row `y`.

        E.g. if sensor is at 43,132 with radius 3 and y is 133, we return columns (10, 11, 12, 13, 14):

               1
               2
               #
              ###
             #####
        132 ###S###
        133  -----
              ###
               #
         */
        let dy = (self.loc.y - y).abs(); // 1
        let dx = Ord::max(0, self.radius - dy); // 3 - 1 = 2
        let start = self.loc.x - dx; // 12 - 2 = 10
        let end = self.loc.x + dx; // 12 + 2 = 14
        start..end + 1 // 10..15 (15 excluded)
    }
}

fn reduce(views: Vec<Range<i32>>) -> Vec<Range<i32>> {
    // Convert a list of ranges to a version without overlaps.

    if views.len() <= 1 {
        return views;
    }

    // Sort by ascending range `start`.
    let sorted = views
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.start, &b.start))
        .collect::<Vec<Range<i32>>>();

    // Start from the left-most range, then push or merge following ranges.

    let first = sorted.first().unwrap();

    sorted
        .iter()
        .skip(1)
        .fold(vec![first.start..first.end], |mut acc, range| {
            let last = acc.last().unwrap();

            let overlaps = range.start <= last.end;

            if overlaps {
                let end = Ord::max(last.end, range.end);
                let merged_range = last.start..end;
                let last_index = acc.len() - 1;
                acc[last_index] = merged_range;
            } else {
                acc.push(range.start..range.end);
            }

            acc
        })
}

fn parse(content: &str) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();

    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    for line in content.lines() {
        let cap = re.captures(line).unwrap();

        let loc = Point2D::new(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );

        let closest_beacon = Point2D::new(
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );

        sensors.push(Sensor::new(loc, closest_beacon));
    }

    sensors
}
