use reader;

fn line_score_part_one(s: &str) -> usize {
    let (a, b) = s.split_once(" ").unwrap();

    /*
    Rock     - A - X
    Paper    - B - Y
    Scissors - C - Z
    */
    match (a, b) {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        (_, _) => 0,
    }
}

fn line_score_part_two(s: &str) -> usize {
    let (a, b) = s.split_once(" ").unwrap();

    match (a, b) {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,
        (_, _) => 0,
    }
}

fn part_one(input: reader::Reader) -> usize {
    input.lines().iter().map(|l| line_score_part_one(l)).sum()
}

fn part_two(input: reader::Reader) -> usize {
    input.lines().iter().map(|l| line_score_part_two(l)).sum()
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
    assert_eq!(part_one(get_test_input()), 15);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 13675);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 12);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 14184);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
