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
            let (_, n) = s.split_once(" ").unwrap();
            let num = n.parse::<isize>().unwrap();
            Ok(Instruction::Addx(num))
        }
    }
}

fn get_cycle_changes(input: reader::Reader) -> Vec<isize> {
    input
        .lines_as::<Instruction>()
        .iter()
        .map(|value| match value {
            Instruction::Noop => vec![0],
            Instruction::Addx(x) => vec![0, x.clone()],
        })
        .flatten()
        .collect::<Vec<isize>>()
}

fn part_one(input: reader::Reader) -> isize {
    let cycles = get_cycle_changes(input);

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

fn part_two(input: reader::Reader) -> String {
    let cycles = get_cycle_changes(input);

    let mut res: Vec<Vec<char>> = vec![];
    let mut x = 1;

    for row in 0..6 {
        let mut line = vec![];
        for col in 0..40 {
            if col - 1 == x || col == x || col + 1 == x {
                line.push('#');
            } else {
                line.push('.');
            }
            x += cycles[(row * 40 + col) as usize];
        }
        res.push(line);
    }

    res.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    println!(
        "Day x\n\tPart 1: {:?}\n\tPart 2: \n{}",
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

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 12460);
}

#[test]
fn test_part_two_example() {
    assert_eq!(
        part_two(get_test_input()),
        r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}

#[test]
fn test_part_two() {
    assert_eq!(
        part_two(input()),
        r"####.####.####.###..###...##..#..#.#....
#.......#.#....#..#.#..#.#..#.#.#..#....
###....#..###..#..#.#..#.#..#.##...#....
#.....#...#....###..###..####.#.#..#....
#....#....#....#....#.#..#..#.#.#..#....
####.####.#....#....#..#.#..#.#..#.####."
    );
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
