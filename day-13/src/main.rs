use reader;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;
use std::{str::FromStr, string::ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    List(VecDeque<Value>),
    Integer(isize),
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self::list(&mut str.chars().into_iter().peekable()))
    }
}

impl Value {
    fn list(iter: &mut Peekable<Chars>) -> Value {
        let mut values = VecDeque::new();

        iter.next();

        loop {
            match iter.peek().unwrap().clone() {
                ',' => {
                    iter.next();
                }
                '0'..='9' => {
                    values.push_back(Self::integer(iter));
                }
                '[' => values.push_back(Self::list(iter)),
                ']' => {
                    iter.next();
                    break;
                }
                value => panic!("Unexpected char: {:?}", value),
            };
        }

        Value::List(values)
    }

    fn integer(iter: &mut Peekable<Chars>) -> Value {
        let mut num_s = String::new();

        while iter.peek().unwrap().is_numeric() {
            num_s.push(iter.next().unwrap());
        }

        Value::Integer(num_s.parse::<isize>().unwrap())
    }
}

fn order(left: &mut Value, right: &mut Value) -> Ordering {
    match (left, right) {
        (Value::Integer(a), Value::Integer(b)) => (*a).cmp(b),
        (Value::List(a), Value::List(b)) => {
            let mut l;
            let mut r;

            while a.len() > 0 || b.len() > 0 {
                l = a.pop_front().unwrap_or(Value::Integer(-1));
                r = b.pop_front().unwrap_or(Value::Integer(-1));

                let res = order(&mut l, &mut r);

                if res != Ordering::Equal {
                    return res;
                }
            }

            Ordering::Equal
        }
        (Value::Integer(a), b) => order(
            &mut Value::List(VecDeque::from(vec![Value::Integer(*a)])),
            b,
        ),
        (a, Value::Integer(b)) => order(
            a,
            &mut Value::List(VecDeque::from(vec![Value::Integer(*b)])),
        ),
        _ => Ordering::Equal,
    }
}

fn part_one(input: reader::Reader) -> usize {
    let mut pairs = input
        .split_on_empty_line()
        .iter()
        .map(|line| {
            line.split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|inner| inner.parse::<Value>().unwrap_or(Value::Integer(0)))
                .collect()
        })
        .collect::<Vec<Vec<Value>>>();

    let mut sum = 0;

    for (index, pair) in pairs.iter_mut().enumerate() {
        let mut r = pair.pop().unwrap();
        let mut l = pair.pop().unwrap();
        if order(&mut l, &mut r) == Ordering::Less {
            sum += 1 + index;
        }
    }

    sum
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
    assert_eq!(part_one(get_test_input()), 13);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 6415);
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
