use reader;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{str::FromStr, string::ParseError};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow: isize,
    tunnels: Vec<String>,
    len_to: HashMap<String, isize>,
}

impl FromStr for Valve {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let s = str.split(" ").collect::<Vec<&str>>();

        Ok(Valve {
            name: s[1].to_string(),
            flow: s[4]
                .replace("rate=", "")
                .replace(";", "")
                .parse::<isize>()
                .unwrap(),
            tunnels: s[9..]
                .iter()
                .map(|valve| valve.replace(",", "").to_string())
                .collect::<Vec<String>>(),
            len_to: HashMap::new(),
        })
    }
}

#[derive(Debug)]
struct Graph {
    valves: Vec<Valve>,
    map: HashMap<String, Valve>,
}

impl Graph {
    fn from(input: reader::Reader) -> Graph {
        let mut valves = input.lines_as::<Valve>();

        // For each valve calculate path length to every other valve
        let mut bfs_map = HashMap::new();
        for valve in valves.iter() {
            bfs_map.insert(valve.name.clone(), valve.clone());
        }

        for mut valve in valves.iter_mut() {
            Self::bfs(&mut valve, &bfs_map);
        }

        // Create final map of valves
        let mut map = HashMap::new();

        for valve in valves.iter() {
            map.insert(valve.name.clone(), valve.clone());
        }

        Graph { valves, map }
    }

    fn bfs(valve: &mut Valve, map: &HashMap<String, Valve>) {
        let mut q: VecDeque<(isize, String)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        q.push_back((0, valve.name.clone()));
        visited.insert(valve.name.clone());

        while !q.is_empty() {
            let (len, v) = q.pop_front().unwrap();
            let v_valve = map.get(&v).unwrap();

            v_valve.tunnels.iter().for_each(|tunnel| {
                if !visited.contains(tunnel) {
                    let curr = map.get(tunnel).unwrap();

                    // It seems like there would be no reason to
                    // have paths to damaged/jammed valves
                    if curr.flow != 0 {
                        valve.len_to.insert(tunnel.clone(), len + 1);
                    }

                    visited.insert(tunnel.clone());
                    q.push_back((len + 1, tunnel.clone()));
                }
            });
        }
    }

    fn max_flow(&self, current: String, mut visited: HashSet<String>, mut time: isize) -> isize {
        let curr = self.map.get(&current).unwrap();
        visited.insert(current.clone());

        let mut flow = 0;

        if curr.flow > 0 {
            time -= 1;
            flow += curr.flow * time;
        }

        let mut m: isize = 0;
        for (dest, len) in curr.len_to.iter() {
            if time - len >= 2 && !visited.contains(dest) {
                m = max(m, self.max_flow(dest.clone(), visited.clone(), time - len));
            }
        }

        flow + m
    }
}

fn part_one(input: reader::Reader) -> isize {
    let g = Graph::from(input);
    g.max_flow("AA".to_string(), HashSet::new(), 30)
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
    assert_eq!(part_one(get_test_input()), 1651);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 1896);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 1707);
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
