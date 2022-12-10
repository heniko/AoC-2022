use reader;
use std::{str::FromStr, string::ParseError};

enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Instruction::Noop)
        } else {
            let (i, n) = s.split_once(" ").unwrap();
            let num = n.parse::<isize>().unwrap();
            Ok(Instruction::Addx(num))
        }
    }
}

fn part_one(input: reader::Reader) -> isize {
    let cycles = input
        .lines_as::<Instruction>()
        .iter()
        .map(|value| match value {
            Instruction::Noop => vec![0],
            Instruction::Addx(x) => vec![0, x.clone()],
        })
        .flat_map(|value| value)
        .collect::<Vec<isize>>();

    let mut res = 0;
    let mut x = 1;

    for i in 0..cycles.len() {
        let cycle = i as isize + 1;

        if (cycle - 20) % 40 == 0 {
            res += cycle * x;
        }

        x += cycles[i];
    }

    res
}

fn part_two(input: reader::Reader) -> isize {
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
    assert_eq!(part_one(get_test_input()), 13140);
}

#[ignore]
#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 0);
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
