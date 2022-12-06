use reader;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

struct CargoCrane {
    crates: Vec<Vec<String>>,
}

impl CargoCrane {
    fn move_crate(&mut self, from: usize, to: usize) {
        let c = self.crates[from].pop();
        match c {
            Some(value) => {
                self.crates[to].push(value);
            }
            None => {
                self.print_crates();
                panic!("From: {}, To: {}", from, to);
            }
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.amount {
            self.move_crate(instruction.from, instruction.to);
        }
    }

    fn get_top_row(&mut self) -> String {
        let mut res = String::new();

        for i in 1..self.crates.len() {
            let top = self.crates[i].pop();

            if let Some(text) = top {
                res = format!("{}{}", res, text);
            }
        }

        res
    }

    fn print_crates(&self) {
        println!("{:?}", &self.crates);
    }
}

fn parse_crates(crate_input: String) -> Vec<Vec<String>> {
    let mut crate_lines = crate_input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    dbg!("{}", crate_lines.pop()); // Remove crate index line

    let crate_line_pattern = Regex::new(r"(^| )(   |\[\w\])").unwrap();

    let mut crates_parsed = crate_lines
        .iter()
        .map(|line| {
            crate_line_pattern
                .captures_iter(line)
                .map(|capture| capture[2][1..2].to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    crates_parsed.reverse();

    let mut crates: Vec<Vec<String>> = vec![vec![]; crates_parsed[0].len() + 1];

    for line in crates_parsed.iter() {
        for i in 0..crates_parsed[0].len() {
            if line[i] != " ".to_string() {
                crates[i + 1].push(line[i].to_owned());
            }
        }
    }

    crates
}

fn parse_instructions(instruction_input: String) -> Vec<Instruction> {
    let instruction_lines = instruction_input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let instruction_line_pattern = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();

    instruction_lines
        .iter()
        .map(|line| {
            let cap = instruction_line_pattern.captures(line).unwrap();
            return Instruction {
                amount: cap[1].parse::<usize>().unwrap(),
                from: cap[2].parse::<usize>().unwrap(),
                to: cap[3].parse::<usize>().unwrap(),
            };
        })
        .collect::<Vec<Instruction>>()
}

fn part_one(input: reader::Reader) -> String {
    let mut input_divided = input.split_on_empty_line();

    let instructions = parse_instructions(input_divided.pop().unwrap());

    let mut cg = CargoCrane {
        crates: parse_crates(input_divided.pop().unwrap()),
    };

    for ins in instructions.iter() {
        cg.run_instruction(ins);
    }

    cg.get_top_row()
}

fn part_two(input: reader::Reader) -> String {
    "".to_string()
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
    assert_eq!(part_one(get_test_input()), "CMZ".to_string());
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), "ZWHVFWQWW".to_string());
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), "".to_string());
}

#[ignore]
#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), "".to_string());
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
