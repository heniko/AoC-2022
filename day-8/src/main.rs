use reader;

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

fn highest_value_table(line: &Vec<usize>, rev: bool) -> Vec<usize> {
    let mut res = vec![];
    let mut highest = 0;

    line.iter().for_each(|value| {
        res.push(highest);
        if value > &highest {
            highest = value.clone();
        }
    });

    if rev {
        res.reverse();
    }

    res
}

fn part_one(input: reader::Reader) -> usize {
    let rows = parse_input(input);
    let columns = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let rows_highest_from_left = rows
        .iter()
        .map(|row| highest_value_table(row, false))
        .collect::<Vec<Vec<usize>>>();
    let rows_highest_from_right = rows
        .iter()
        .map(|row| {
            highest_value_table(
                &(row
                    .iter()
                    .rev()
                    .map(|value| value.to_owned())
                    .collect::<Vec<usize>>()),
                true,
            )
        })
        .collect::<Vec<Vec<usize>>>();

    let columns_highest_from_left = columns
        .iter()
        .map(|columns| highest_value_table(columns, false))
        .collect::<Vec<Vec<usize>>>();
    let columns_highest_from_right = columns
        .iter()
        .map(|col| {
            highest_value_table(
                &(col
                    .iter()
                    .rev()
                    .map(|value| value.to_owned())
                    .collect::<Vec<usize>>()),
                true,
            )
        })
        .collect::<Vec<Vec<usize>>>();

    let mut res = 0;
    for row in 1..rows.len() - 1 {
        for col in 1..columns.len() - 1 {
            let curr = rows[row][col];
            if curr > rows_highest_from_left[row][col]
                || curr > rows_highest_from_right[row][col]
                || curr > columns_highest_from_left[col][row]
                || curr > columns_highest_from_right[col][row]
            {
                res += 1;
            }
        }
    }

    res + 2 * rows.len() + 2 * columns.len() - 4
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

#[ignore]
#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 0);
}

#[cfg(test)]
fn get_test_input() -> reader::Reader {
    reader::open("input_example.txt")
}
