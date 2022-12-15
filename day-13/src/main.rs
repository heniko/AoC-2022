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
    /*
    Finds the order of two lists recursively.
     */
    match (left, right) {
        // Case: Both values are integers
        (Value::Integer(a), Value::Integer(b)) => (*a).cmp(b),
        // Case: Both values are lists
        (Value::List(a), Value::List(b)) => {
            /*
            While either list contains a value we compare the values
            untill we find one that isn't equal. If list runs out of
            elements default value of -1 is returned meaning that list
            win the next comparison.
            */
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

            // Default case when both lists run out of elements at the same time
            Ordering::Equal
        }
        // Case: Left = Integer, Right = List
        (Value::Integer(a), b) => order(
            &mut Value::List(VecDeque::from(vec![Value::Integer(*a)])),
            b,
        ),
        // Case: Left = List, Right = Integer
        (a, Value::Integer(b)) => order(
            a,
            &mut Value::List(VecDeque::from(vec![Value::Integer(*b)])),
        ),
    }
}

fn part_one(input: reader::Reader) -> usize {
    // Split input into pairs of values
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

    // Compare pairs and sum indexes where pairs are in
    // correct order (With +1 offset).
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
    let divider_packets = vec!["[[2]]".to_string(), "[[6]]".to_string()];

    // Split input to lines, add divider packets, remove
    // empty lines and parse lines as Value type.
    let mut values = input
        .lines()
        .iter()
        .chain(divider_packets.iter())
        .filter(|line| line.as_str() != "")
        .map(|line| line.parse::<Value>().unwrap())
        .collect::<Vec<Value>>();

    // Sort with order(). Values are cloned as order() mutates them.
    values.sort_by(|a, b| order(&mut a.clone(), &mut b.clone()));

    let mut first = 0;
    let mut second = 0;

    // Find indexes of divider packets
    for (index, value) in values.iter().enumerate() {
        if value
            == &Value::List(VecDeque::from(vec![Value::List(VecDeque::from(vec![
                Value::Integer(2),
            ]))]))
        {
            first = index;
        }

        if value
            == &Value::List(VecDeque::from(vec![Value::List(VecDeque::from(vec![
                Value::Integer(6),
            ]))]))
        {
            second = index;
        }
    }

    (first + 1) * (second + 1)
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
    assert_eq!(part_two(get_test_input()), 140);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 20056);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
