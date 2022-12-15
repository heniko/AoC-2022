use reader;
use std::cmp::min;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn from(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

struct Heightmap {
    start: Position,
    end: Position,
    map: Vec<Vec<i32>>,
    height: usize,
    width: usize,
}

impl Heightmap {
    fn from(input: reader::Reader) -> Heightmap {
        let (start, end, map) = Heightmap::parse_map(input);

        Heightmap {
            start,
            end,
            height: map.len(),
            width: map[0].len(),
            map,
        }
    }

    fn parse_map(input: reader::Reader) -> (Position, Position, Vec<Vec<i32>>) {
        let chars: Vec<Vec<char>> = input
            .lines()
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let mut map: Vec<Vec<i32>> = vec![];
        let mut start = Position::from(0, 0);
        let mut end = Position::from(0, 0);

        for x in 0..chars.len() {
            map.push(vec![]);
            for y in 0..chars[0].len() {
                let char = match chars[x][y] {
                    'S' => {
                        start = Position::from(x, y);
                        'a'
                    }
                    'E' => {
                        end = Position::from(x, y);
                        'z'
                    }
                    value => value,
                };

                map[x].push(char as i32 - 'a' as i32);
            }
        }

        (start, end, map)
    }

    fn get_adjacency_list(&self, pos: Position) -> Vec<Position> {
        let mut res = vec![];

        // Up
        if pos.x > 0 {
            res.push(Position::from(pos.x - 1, pos.y));
        }
        // Down
        if pos.x < self.height - 1 {
            res.push(Position::from(pos.x + 1, pos.y));
        }
        // Left
        if pos.y > 0 {
            res.push(Position::from(pos.x, pos.y - 1));
        }
        // Right
        if pos.y < self.width - 1 {
            res.push(Position::from(pos.x, pos.y + 1));
        }

        res.into_iter()
            .filter(|elem| self.get_height(*elem) - self.get_height(pos) <= 1)
            .collect()
    }

    fn get_height(&self, pos: Position) -> i32 {
        self.map[pos.x][pos.y]
    }

    fn bfs(&self) -> i32 {
        let mut q = VecDeque::new();
        let mut explored = vec![vec![false; self.width]; self.height];

        explored[self.start.x][self.start.y] = true;
        q.push_back((0, self.start));

        while q.len() > 0 {
            let (len, curr) = q.pop_front().unwrap();

            if self.end.x == curr.x && self.end.y == curr.y {
                return len;
            }

            self.get_adjacency_list(curr).iter().for_each(|pos| {
                if !explored[pos.x][pos.y] {
                    q.push_back((len + 1, *pos));
                    explored[pos.x][pos.y] = true;
                }
            })
        }

        std::i32::MAX
    }
}

fn part_one(input: reader::Reader) -> i32 {
    Heightmap::from(input).bfs()
}

fn part_two(input: reader::Reader) -> i32 {
    let mut map = Heightmap::from(input);

    let mut shortest = map.bfs();

    for x in 0..map.map.len() {
        for y in 0..map.map[0].len() {
            if map.get_height(Position::from(x, y)) == 0 {
                map.start = Position::from(x, y);
                let len = map.bfs();
                shortest = min(shortest, len)
            }
        }
    }

    shortest
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
    assert_eq!(part_one(get_test_input()), 31);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 520);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 29);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 508);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
