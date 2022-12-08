use reader;

fn parse_folder_sizes(lines: &mut Vec<String>, res: &mut Vec<usize>) -> usize {
    let mut current_size = 0;

    loop {
        let l = lines.pop().unwrap_or("$ cd ..".to_string());

        if l.starts_with(&"$ cd ..") {
            break;
        } else if l.starts_with("$ cd") {
            current_size += parse_folder_sizes(lines, res);
        } else if l.as_bytes()[0].is_ascii_digit() {
            current_size += l.split(" ").collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();
        }
    }

    res.push(current_size);
    current_size
}

fn part_one(input: reader::Reader) -> usize {
    let mut res: Vec<usize> = vec![];
    let mut lines = input.lines();
    lines.reverse();
    parse_folder_sizes(&mut lines, &mut res);

    res.iter().filter(|value| value <= &&100_000).sum()
}

fn part_two(input: reader::Reader) -> usize {
    let mut res: Vec<usize> = vec![];
    let mut lines = input.lines();
    lines.reverse();
    parse_folder_sizes(&mut lines, &mut res);
    let space_needed = 30_000_000 - (70_000_000 - res.last().unwrap());

    res.iter()
        .filter(|value| value >= &&space_needed)
        .min()
        .unwrap()
        .clone()
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
    assert_eq!(part_one(get_test_input()), 95437);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 1086293);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 24933642);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 366028);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
