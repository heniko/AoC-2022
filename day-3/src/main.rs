use reader;

fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 1 + 26,
        _ => 0,
    }
}

fn line_priority(s: &str) -> usize {
    let first = &s[..s.len() / 2];
    let second = &s[s.len() / 2..];

    let first_chars = first.chars().collect::<Vec<char>>();
    let second_chars = second.chars().collect::<Vec<char>>();

    for ch in first_chars {
        if second_chars.iter().any(|&i| i == ch) {
            return to_priority(ch);
        }
    }

    0
}

fn part_one(input: reader::Reader) -> usize {
    input
        .lines()
        .iter()
        .map(|line| line_priority(line))
        .collect::<Vec<usize>>()
        .iter()
        .sum()
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
    assert_eq!(part_one(get_test_input()), 157);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 70);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 7845);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 0);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
