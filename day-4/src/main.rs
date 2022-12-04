use reader;
use std::{str::FromStr, string::ParseError};

#[derive(Clone, Debug, Copy)]
struct Range {
    begin: usize,
    end: usize,
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (begin, end) = str.split_once("-").unwrap();

        Ok(Self {
            begin: begin.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
        })
    }
}

fn get_ranges(input: reader::Reader) -> Vec<Vec<Range>> {
    input
        .lines()
        .iter()
        .map(|line| {
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|range| range.parse::<Range>().unwrap())
                .collect::<Vec<Range>>()
        })
        .collect()
}

fn overlaps_part_one(a: Range, b: Range) -> usize {
    if a.begin <= b.begin && a.end >= b.end {
        1
    } else if b.begin <= a.begin && b.end >= a.end {
        1
    } else {
        0
    }
}

fn overlaps_part_two(a: Range, b: Range) -> usize {
    if a.end >= b.begin && a.begin <= b.end {
        1
    } else if b.end >= a.begin && b.begin <= a.end {
        1
    } else {
        0
    }
}

fn part_one(input: reader::Reader) -> usize {
    get_ranges(input)
        .iter()
        .map(|elem| overlaps_part_one(elem[0], elem[1]))
        .sum()
}

fn part_two(input: reader::Reader) -> usize {
    get_ranges(input)
        .iter()
        .map(|elem| overlaps_part_two(elem[0], elem[1]))
        .sum()
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
    assert_eq!(part_one(get_test_input()), 2);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 532);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 4);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 854);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
