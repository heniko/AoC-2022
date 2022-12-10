use reader;
use std::cmp::max;

fn parse_input(input: reader::Reader) -> Vec<Vec<usize>> {
    input
        .lines()
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| (char.to_string()).parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn two_way_score(
    lines: &Vec<Vec<usize>>,
    f: fn(Vec<usize>) -> Vec<usize>,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut left = vec![];
    let mut right = vec![];

    lines.iter().for_each(|line| {
        // Calculate row score (left)
        let mut r = line.clone();
        r = f(r);
        // Calculate reversed row score (right)
        let mut rr = line.clone();
        rr.reverse();
        rr = f(rr);
        rr.reverse();

        left.push(r);
        right.push(rr);
    });

    (left, right)
}

fn directed_score_maps(
    rows: &Vec<Vec<usize>>,
    f: fn(Vec<usize>) -> Vec<usize>,
) -> (
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
) {
    // https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
    let columns = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let (left, right) = two_way_score(&rows, f);
    let (down, up) = two_way_score(&columns, f);

    (up, down, left, right)
}

// Score function for part 1
fn max_heights(line: Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    let mut highest = 0;

    line.iter().for_each(|value| {
        res.push(highest);
        if value > &highest {
            highest = value.clone();
        }
    });

    res
}

// Score function for part 2
fn directed_scenic_score(line: Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];

    for i in 1..line.len() {
        let mut score = 0;

        for j in (0..i).rev() {
            score += 1;
            if line[j] >= line[i] {
                break;
            }
        }

        res.push(score);
    }

    res
}

fn part_one(input: reader::Reader) -> usize {
    let rows = parse_input(input);

    let (up, down, left, right) = directed_score_maps(&rows, max_heights);

    let mut res = 0;
    for row in 1..rows.len() - 1 {
        for col in 1..rows[0].len() - 1 {
            let curr = rows[row][col];
            if curr > left[row][col]
                || curr > right[row][col]
                || curr > down[col][row]
                || curr > up[col][row]
            {
                res += 1;
            }
        }
    }

    res + 2 * rows.len() + 2 * rows[0].len() - 4
}

fn part_two(input: reader::Reader) -> usize {
    let rows = parse_input(input);

    let (up, down, left, right) = directed_score_maps(&rows, directed_scenic_score);

    let mut highest = 0;
    for row in 1..rows.len() - 1 {
        for col in 1..rows[0].len() - 1 {
            let curr = up[col][row] * down[col][row] * left[row][col] * right[row][col];
            highest = max(highest, curr);
        }
    }

    highest
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
    assert_eq!(part_one(get_test_input()), 21);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 1705);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 8);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 371200);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
