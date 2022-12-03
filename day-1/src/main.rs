use reader;

fn summed_food(input: reader::Reader) -> Vec<usize> {
    input
        .split_on_empty_line()
        .iter()
        .map(|value| value.lines().map(|num| num.parse::<usize>().unwrap()).sum())
        .collect()
}

fn part_one(input: reader::Reader) -> usize {
    summed_food(input).iter().max().unwrap().clone()
}

fn part_two(input: reader::Reader) -> usize {
    let mut elves = summed_food(input);
    elves.sort();
    elves.reverse();
    let top_three = &elves[0..3];
    top_three.iter().sum()
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
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 24_000);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 45_000);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
