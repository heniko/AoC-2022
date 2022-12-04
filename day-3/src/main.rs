use reader;

fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 1 + 26,
        _ => panic!("Invalid character: {}", c),
    }
}

fn find_common(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    a.into_iter().filter(|elem| b.contains(elem)).collect()
}

fn part_one(input: reader::Reader) -> usize {
    input
        .lines()
        .iter()
        .map(|elem| {
            find_common(
                elem[..elem.len() / 2].chars().collect::<Vec<char>>(),
                elem[elem.len() / 2..].chars().collect::<Vec<char>>(),
            )
        })
        .map(|elem| to_priority(elem[0]))
        .sum()
}

fn part_two(input: reader::Reader) -> usize {
    input
        .lines()
        .chunks(3)
        .map(|chunk| {
            find_common(
                chunk[0].chars().collect::<Vec<char>>(),
                find_common(
                    chunk[1].chars().collect::<Vec<char>>(),
                    chunk[2].chars().collect::<Vec<char>>(),
                ),
            )
        })
        .map(|elem| to_priority(elem[0]))
        .sum()
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
    assert_eq!(part_two(input()), 2790);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
