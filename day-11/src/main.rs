use reader;
use std::{str::FromStr, string::ParseError};

enum Num {
    Old,
    Num(usize),
}

impl Num {
    fn get_value(&self, old: usize) -> usize {
        match self {
            Num::Old => old,
            Num::Num(value) => *value,
        }
    }
}

struct Op {
    left: Num,
    right: Num,
    op: String,
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split(" ").collect::<Vec<&str>>();

        let left = Op::num_or_old(ss[0]);
        let right = Op::num_or_old(ss[2]);

        Ok(Op {
            left,
            right,
            op: ss[1].to_string(),
        })
    }
}

impl Op {
    fn num_or_old(s: &str) -> Num {
        match s {
            "old" => Num::Old,
            _ => Num::Num(s.parse().unwrap()),
        }
    }

    fn run(&self, old: usize) -> usize {
        let l = self.left.get_value(old);
        let r = self.right.get_value(old);

        match self.op.as_str() {
            "*" => l * r,
            "+" => l + r,
            _ => panic!("Unknown op"),
        }
    }
}

struct Test {
    if_true: usize,
    if_false: usize,
    modulo: usize,
}

impl Test {
    fn run(&self, item: usize) -> usize {
        match item % self.modulo {
            0 => self.if_true,
            _ => self.if_false,
        }
    }
}

struct Monkey {
    op: Op,
    test: Test,
    items: Vec<usize>,
    count: usize,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        iter.next();

        let items = iter.next().unwrap()[18..]
            .split(", ")
            .map(|value| value.parse::<usize>().unwrap())
            .collect();

        let op = iter.next().unwrap()[19..].parse::<Op>().unwrap();

        let test = Test {
            modulo: iter.next().unwrap()[21..].parse().unwrap(),
            if_true: iter.next().unwrap()[29..].parse().unwrap(),
            if_false: iter.next().unwrap()[30..].parse().unwrap(),
        };

        Ok(Monkey {
            op,
            test,
            items,
            count: 0,
        })
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    relaxer: Box<dyn Fn(usize) -> usize>,
}

impl Monkeys {
    fn from(monkeys: Vec<Monkey>, relaxer: Box<dyn Fn(usize) -> usize>) -> Monkeys {
        Monkeys { monkeys, relaxer }
    }

    fn round(&mut self) {
        for index in 0..self.monkeys.len() {
            let mut monkey = &mut self.monkeys[index];

            let to_add = monkey
                .items
                .iter()
                .map(|item| monkey.op.run(*item))
                .map(|item| self.relaxer.as_ref()(item))
                .map(|item| (monkey.test.run(item), item))
                .collect::<Vec<(usize, usize)>>();

            monkey.items.clear();
            monkey.count += to_add.len();

            to_add
                .iter()
                .for_each(|(to, item)| self.monkeys[*to].items.push(*item));
        }
    }

    fn get_level_of_monkey_business(&mut self) -> usize {
        self.monkeys.sort_by(|a, b| b.count.cmp(&a.count));
        self.monkeys
            .iter()
            .map(|monkey| monkey.count)
            .take(2)
            .product()
    }
}

fn part_one(input: reader::Reader) -> usize {
    let parsed_input = input.split_on_empty_line_into::<Monkey>();
    let mut monkeys = Monkeys::from(parsed_input, Box::from(|x| x / 3));

    (0..20).for_each(|_| monkeys.round());

    monkeys.get_level_of_monkey_business()
}

fn part_two(input: reader::Reader) -> usize {
    let parsed_input = input.split_on_empty_line_into::<Monkey>();
    let modulo: usize = parsed_input
        .iter()
        .map(|monkey| monkey.test.modulo)
        .product();
    let mut monkeys = Monkeys::from(parsed_input, Box::from(move |x| x % modulo));

    (0..10000).for_each(|_| monkeys.round());

    monkeys.get_level_of_monkey_business()
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

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 30_599_555_965);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
