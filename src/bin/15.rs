use itertools::Itertools;
use regex::Regex;
use std::ops::Range;

// Beacon Exclusion Zone
// https://adventofcode.com/2022/day/15

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<usize> {
    let sensors = parse(input);
    Some(count_beacon_forbidden(&sensors, 2000000))
}

fn part2(input: &str) -> Option<i64> {
    let sensors = parse(input);
    Some(find_distress_signal_frequency(&sensors, 4000000))
}

fn count_beacon_forbidden(sensors: &Vec<Sensor>, y: i64) -> usize {
    let views = sensors.iter().map(|s| s.get_view(y)).collect();

    reduce(views)
        .iter()
        .map(|range| (range.end - range.start) as usize)
        .sum::<usize>()
        - 1 // 1 beacon must be on this row
}

fn find_distress_signal_frequency(sensors: &Vec<Sensor>, search_area_size: i64) -> i64 {
    /*
    If there is a unique position for the distress signal D,
    it must be somewhere just outside each sensor's 2D range:

      #
     ###D
    ##S##
     ###
      #

    So, we build the set of all points that define a sensor's outer boundary,
    then look for the one that's not in the range of any sensor.
     */

    let signal = sensors
        .iter()
        .flat_map(|s| s.get_outer_boundary(0..search_area_size))
        .filter(|candidate| sensors.iter().all(|s| !s.contains(candidate)))
        .next()
        .unwrap();

    signal.x * 4000000 + signal.y
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Point2D) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

struct Sensor {
    loc: Point2D,
    radius: i64,
}

impl Sensor {
    fn new(loc: Point2D, closest_beacon: Point2D) -> Self {
        let radius = loc.dist(&closest_beacon);
        Self { loc, radius }
    }

    fn get_view(&self, y: i64) -> Range<i64> {
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

    fn get_outer_boundary(&self, search_area: Range<i64>) -> Vec<Point2D> {
        let top = self.loc.y - self.radius - 1;
        let bottom = self.loc.y + self.radius + 1;

        (0..self.radius + 1)
            .flat_map(|k| {
                vec![
                    Point2D::new(self.loc.x + k, top + k),    // top-right line
                    Point2D::new(self.loc.x + k, bottom - k), // bottom-right line
                    Point2D::new(self.loc.x - k, top + k),    // bottom-left line
                    Point2D::new(self.loc.x - k, bottom - k), // top-left line
                ]
            })
            .filter(|p| search_area.contains(&p.x) && search_area.contains(&p.y))
            .collect()
    }

    fn contains(&self, p: &Point2D) -> bool {
        self.loc.dist(&p) <= self.radius
    }
}

fn reduce(views: Vec<Range<i64>>) -> Vec<Range<i64>> {
    // Convert a list of ranges to a version without overlaps.

    if views.len() <= 1 {
        return views;
    }

    // Sort by ascending range `start`.
    let sorted = views
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.start, &b.start))
        .collect::<Vec<Range<i64>>>();

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
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
        );

        let closest_beacon = Point2D::new(
            cap[3].parse::<i64>().unwrap(),
            cap[4].parse::<i64>().unwrap(),
        );

        sensors.push(Sensor::new(loc, closest_beacon));
    }

    sensors
}

#[test]
#[ignore]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 15);
    assert_eq!(part1(input), Some(4876693));
}

#[test]
#[ignore]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 15);
    assert_eq!(part2(input), Some(11645454855041));
}
