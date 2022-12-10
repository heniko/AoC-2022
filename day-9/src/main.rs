use reader;
use std::collections::{HashSet, LinkedList};
use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    fn as_follow_direction(self) -> Position {
        let mut y = self.y;
        let mut x = self.x;

        if self.y > 1 {
            y = self.y - 1;
        } else if self.y < -1 {
            y = self.y + 1;
        }

        if self.x > 1 {
            x = self.x - 1;
        } else if self.x < -1 {
            x = self.x + 1;
        }

        if -2 < self.x && self.x < 2 && -2 < self.y && self.y < 2 {
            x = 0;
            y = 0;
        }

        Position { x, y }
    }
}

struct Rope {
    positions: HashSet<Position>,
    position: Position,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            positions: HashSet::from([Position { x: 0, y: 0 }]),
            position: Position { x: 0, y: 0 },
        }
    }

    fn from(length: usize) -> LinkedList<Rope> {
        let mut res = LinkedList::new();

        (0..length).for_each(|_| res.push_back(Rope::new()));

        res
    }

    fn move_to_dir(&mut self, dir: Position) {
        self.position = self.position + dir;
        self.positions.insert(self.position);
    }

    fn move_towards(&mut self, pos: Position) {
        let dir = (pos - self.position).as_follow_direction();
        self.move_to_dir(dir);
    }
}

fn move_rope(rope: &mut LinkedList<Rope>, dir: Position) {
    rope.front_mut().unwrap().move_to_dir(dir);
    let mut target = rope.front().unwrap().position;

    rope.iter_mut().skip(1).for_each(|r| {
        r.move_towards(target);
        target = r.position;
    })
}

fn execute_lines(rope: &mut LinkedList<Rope>, lines: &Vec<String>) {
    lines.iter().for_each(|line| {
        let (dir, amount) = line.split_once(" ").unwrap();

        let dir = match dir {
            "U" => Position { x: 0, y: 1 },
            "D" => Position { x: 0, y: -1 },
            "L" => Position { x: -1, y: 0 },
            "R" => Position { x: 1, y: 0 },
            _ => Position { x: 0, y: 0 },
        };

        for _ in 0..amount.parse::<usize>().unwrap() {
            move_rope(rope, dir);
        }
    })
}

fn part_one(input: reader::Reader) -> usize {
    let lines = input.lines();
    let mut rope = Rope::from(2);
    execute_lines(&mut rope, &lines);

    rope.back().unwrap().positions.len()
}

fn part_two(input: reader::Reader) -> usize {
    let lines = input.lines();
    let mut rope = Rope::from(10);
    execute_lines(&mut rope, &lines);

    rope.back().unwrap().positions.len()
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
    assert_eq!(part_one(get_test_input()), 88); // 13 for the original test data
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 6067);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 36);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 2471);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
