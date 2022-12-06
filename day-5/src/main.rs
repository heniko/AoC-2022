use reader;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

struct CrateMover {
    stacks: Vec<Vec<String>>,
    held_crates: Vec<String>,
}

impl CrateMover {
    fn from(stacks: Vec<Vec<String>>) -> CrateMover {
        CrateMover {
            stacks,
            held_crates: vec![],
        }
    }

    fn move_crates(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            self.held_crates.push(self.stacks[from - 1].pop().unwrap());
        }
        for _ in 0..amount {
            self.stacks[to - 1].push(self.held_crates.pop().unwrap());
        }
    }

    fn read_top_line(&mut self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().clone().unwrap())
            .fold(String::new(), |acc, new| format!("{}{}", acc, new))
    }

    fn execute_version_9000_instruction(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.amount {
            self.move_crates(1, instruction.from, instruction.to);
        }
    }

    fn execute_version_9001_instruction(&mut self, instruction: &Instruction) {
        self.move_crates(instruction.amount, instruction.from, instruction.to);
    }
}

fn parse_crates(crate_input: String) -> Vec<Vec<String>> {
    let mut split_lines = crate_input.lines().collect::<Vec<&str>>();
    split_lines.pop();

    let lines = split_lines
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| chunk[1].to_string())
                .collect::<Vec<String>>()
        })
        .rev()
        .collect::<Vec<Vec<String>>>();

    // https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
    (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<String>>()
        })
        .map(|line| {
            line.into_iter()
                .filter(|elem| elem != &" ".to_string())
                .collect()
        })
        .collect()
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

    let stacks = parse_crates(input_divided.pop().unwrap());
    let mut crane = CrateMover::from(stacks);

    for ins in instructions.iter() {
        crane.execute_version_9000_instruction(ins)
    }

    crane.read_top_line()
}

fn part_two(input: reader::Reader) -> String {
    let mut input_divided = input.split_on_empty_line();
    let instructions = parse_instructions(input_divided.pop().unwrap());

    let stacks = parse_crates(input_divided.pop().unwrap());
    let mut crane = CrateMover::from(stacks);

    for ins in instructions.iter() {
        crane.execute_version_9001_instruction(ins)
    }

    crane.read_top_line()
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
    assert_eq!(part_two(get_test_input()), "MCD".to_string());
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), "HZFZCCWWV".to_string());
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
