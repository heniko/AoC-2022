use reader;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops;
use std::{str::FromStr, string::ParseError};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (x, y) = str.split_once(",").unwrap();
        Ok(Point {
            x: x.parse::<isize>().unwrap(),
            y: y.parse::<isize>().unwrap(),
        })
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Rock {
    points: Vec<Point>,
}

impl FromStr for Rock {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Rock {
            points: str
                .split(" -> ")
                .map(|elem| elem.parse::<Point>().unwrap())
                .collect::<Vec<Point>>(),
        })
    }
}

#[derive(Debug)]
struct Map {
    rocks: Vec<Rock>,
    map: HashSet<Point>,
    max_y: isize,
}

impl Map {
    fn from(input: reader::Reader) -> Map {
        let mut map = Map {
            rocks: input
                .lines()
                .iter()
                .map(|elem| elem.parse::<Rock>().unwrap())
                .collect::<Vec<Rock>>(),
            map: HashSet::new(),
            max_y: 0,
        };

        for rock in map.rocks.iter() {
            for i in 1..rock.points.len() {
                let prev = rock.points[i - 1];
                let curr = rock.points[i];

                // Only one pair of these should be different.
                let min_x = min(prev.x, curr.x);
                let max_x = max(prev.x, curr.x);

                let min_y = min(prev.y, curr.y);
                let max_y = max(prev.y, curr.y);

                // Max y is used to determine if sand has fallen to void.
                map.max_y = max(max_y, map.max_y);

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        map.map.insert(Point { x, y });
                    }
                }
            }
        }

        map
    }

    fn drop_sand(&mut self) -> bool {
        let mut curr = Point { x: 500, y: 0 };

        while curr.y <= self.max_y {
            let down = curr + Point { x: 0, y: 1 };
            let left = curr + Point { x: -1, y: 1 };
            let right = curr + Point { x: 1, y: 1 };

            if self.is_free(&down) {
                curr = down;
            } else if self.is_free(&left) {
                curr = left;
            } else if self.is_free(&right) {
                curr = right;
            } else {
                // Down, left and right filled so place sand here
                self.map.insert(curr);
                return false;
            }
        }

        // While loop completed so void was reached
        true
    }

    fn is_free(&self, p: &Point) -> bool {
        !self.map.contains(p)
    }
}

fn part_one(input: reader::Reader) -> usize {
    let mut map = Map::from(input);

    let mut count = 0;

    while !map.drop_sand() {
        count += 1;
    }

    count
}

fn part_two(input: reader::Reader) -> usize {
    0
}

fn main() {
    println!(
        "Day x\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> reader::Reader {
    reader::open("input.txt")
}

#[test]
fn test_part_one_example() {
    assert_eq!(part_one(get_test_input()), 24);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 665);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 0);
}

#[ignore]
#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 0);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
