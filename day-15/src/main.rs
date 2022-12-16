use reader;
use regex::Regex;
use std::cmp::max;
use std::{str::FromStr, string::ParseError};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn from(start: isize, end: isize) -> Range {
        Range { start, end }
    }

    fn len(&self) -> isize {
        self.end - self.start
    }

    fn common(&self, other_end: isize) -> isize {
        if self.end <= other_end {
            self.len()
        } else if self.start <= other_end {
            // Range end is inclusive
            other_end - self.start
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    at: Point,
    beacon: Point,
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$",
        )
        .unwrap();

        let caps = re.captures(str).unwrap();

        Ok(Sensor {
            at: Point::from(
                caps[1].parse::<isize>().unwrap(),
                caps[2].parse::<isize>().unwrap(),
            ),
            beacon: Point::from(
                caps[3].parse::<isize>().unwrap(),
                caps[4].parse::<isize>().unwrap(),
            ),
        })
    }
}

impl Sensor {
    fn closest_len(&self) -> isize {
        self.at.manhattan_distance(self.beacon)
    }

    fn range_at_row(&self, y: isize) -> Option<Range> {
        // Returns the range of cover.
        let extras = self.closest_len() - self.at.manhattan_distance(Point { x: self.at.x, y });

        if extras < 0 {
            None
        } else {
            Some(Range::from(self.at.x - extras, self.at.x + extras))
        }
    }

    fn is_covering(&self, p: Point) -> bool {
        self.at.manhattan_distance(p) <= self.closest_len()
    }
}

fn part_one(input: reader::Reader, y: isize) -> isize {
    let mut ranges = input
        .lines_as::<Sensor>()
        .iter()
        .map(|sensor| sensor.range_at_row(y))
        .filter(|value| value != &Option::None)
        .map(|value| value.unwrap())
        .collect::<Vec<Range>>();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut len = 0;
    let mut max_end = -2_000_000_000;

    for range in ranges.iter() {
        len += range.len() - range.common(max_end);
        max_end = max(max_end, range.end);
    }

    len
}

fn part_two(input: reader::Reader, y: isize) -> isize {
    let sensors = input.lines_as::<Sensor>();

    let mut res = Point::from(0, 0);

    for i in 0..=y {
        let mut x = 0;
        while x <= y {
            let mut x_xhanged = false;
            for sensor in sensors.iter() {
                if sensor.is_covering(Point::from(x, i)) {
                    x_xhanged = true;
                    x = sensor.range_at_row(i).unwrap().end + 1;
                    break;
                }
            }
            if !x_xhanged {
                break;
            }
        }
        if x <= y {
            res = Point::from(x, i);
            break;
        }
    }

    res.x * 4_000_000 + res.y
}

fn main() {
    println!(
        "Day x\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input(), 2_000_000),
        part_two(input(), 4_000_000)
    );
}

fn input() -> reader::Reader {
    reader::open("input.txt")
}

#[test]
fn test_part_one_example() {
    assert_eq!(part_one(get_test_input(), 10), 26);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input(), 2_000_000), 4876693);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input(), 20), 56000011);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input(), 4_000_000), 11645454855041);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
