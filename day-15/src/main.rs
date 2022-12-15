use reader;

fn part_one(input: reader::Reader) -> usize {
    0
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
    assert_eq!(part_one(get_test_input()), 26);
}

#[ignore]
#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 0);
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
