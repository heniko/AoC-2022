use reader;

fn find_first_subarray_of_uniques(input: reader::Reader, len: usize) -> usize {
    let chars = input.text().chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        let mut s = chars[i..i + len].to_owned();
        s.sort();
        s.dedup();

        if s.len() == len {
            return i + len;
        }
    }

    0
}

fn part_one(input: reader::Reader) -> usize {
    find_first_subarray_of_uniques(input, 4)
}

fn part_two(input: reader::Reader) -> usize {
    find_first_subarray_of_uniques(input, 14)
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
    assert_eq!(part_one(get_test_input()), 7);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 1542);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 19);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 3153);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
