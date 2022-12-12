use reader;
use std::{collections::BinaryHeap, str::FromStr, string::ParseError};

enum Operation {
    Number(i64),
    Old,
    Mul,
    Plus,
}

struct Monkey {
    inspections: i64,
    items: Vec<i64>,
    op: Vec<Operation>,
    modulo: i64,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split("\n").collect::<Vec<&str>>();
        let mut iter = ss.iter();

        iter.next(); // Skip "Monkey x:" line

        let items = iter.next().unwrap()[18..]
            .split(", ")
            .map(|value| value.parse::<i64>().unwrap())
            .collect();

        let op = iter.next().unwrap()[19..]
            .split(" ")
            .map(|value| match value {
                "old" => Operation::Old,
                "*" => Operation::Mul,
                "+" => Operation::Plus,
                _ => Operation::Number(value.parse().unwrap()),
            })
            .collect();

        Ok(Monkey {
            inspections: 0,
            items,
            op,
            modulo: iter.next().unwrap()[21..].parse().unwrap(),
            if_true: iter.next().unwrap()[29..].parse().unwrap(),
            if_false: iter.next().unwrap()[30..].parse().unwrap(),
        })
    }
}

impl Monkey {
    fn inspect_item(&mut self) -> Option<(usize, i64)> {
        let item = self.items.pop();

        match item {
            None => None,
            Some(i) => {
                self.inspections += 1;

                let item_worry_level = self.inspect(i);

                if item_worry_level % self.modulo == 0 {
                    Some((self.if_true, item_worry_level))
                } else {
                    Some((self.if_false, item_worry_level))
                }
            }
        }
    }

    fn take_item(&mut self, item: i64) {
        self.items.push(item);
    }

    fn inspect(&self, item: i64) -> i64 {
        let left = match self.op[0] {
            Operation::Old => item,
            Operation::Number(val) => val,
            _ => panic!("Unexpected op"),
        };

        let right = match self.op[2] {
            Operation::Old => item,
            Operation::Number(val) => val,
            _ => panic!("Unexpected op"),
        };

        let eval = match self.op[1] {
            Operation::Mul => left * right,
            Operation::Plus => left + right,
            _ => panic!("Unexpected op"),
        };

        eval / 3
    }
}

fn part_one(input: reader::Reader) -> i64 {
    let mut monkeys = input.split_on_empty_line_into::<Monkey>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                let res = monkeys[i].inspect_item();

                match res {
                    None => break,
                    Some((to, item)) => monkeys[to].take_item(item),
                }
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<BinaryHeap<i64>>()
        .iter()
        .take(2)
        .fold(1, |acc, val| acc * val)
}

fn part_two(input: reader::Reader) -> i64 {
    let mut monkeys = input.split_on_empty_line_into::<Monkey>();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            loop {
                let res = monkeys[i].inspect_item();

                match res {
                    None => break,
                    Some((to, item)) => monkeys[to].take_item(item),
                }
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<BinaryHeap<i64>>()
        .iter()
        .take(2)
        .fold(1, |acc, val| acc * val)
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
    assert_eq!(part_one(get_test_input()), 10605);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 113220);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 2_713_310_158);
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
