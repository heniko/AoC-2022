use reader;

#[derive(Debug, Clone)]
enum FileSystem {
    File {
        size: usize,
        name: String,
    },
    Folder {
        files: Vec<FileSystem>,
        name: String,
        size: usize,
    },
}

fn parse_folder(logs: &mut Vec<String>, name: String) -> FileSystem {
    // Recursively parse file system contents
    let mut current_files: Vec<FileSystem> = vec![];
    let mut total_size: usize = 0;

    loop {
        let line = logs.pop();

        if let Some(value) = line {
            if value.starts_with("$ cd ..") {
                break;
            } else if value.starts_with("$ cd") {
                // Recursively parse subfolder
                let mut folder_name = value
                    .split(" ")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>();
                let f = parse_folder(logs, folder_name.pop().unwrap());

                if let FileSystem::Folder { files, name, size } = f.clone() {
                    total_size += size;
                    current_files.push(f);
                } else {
                    panic!("Not a folder: {:?}", f);
                }
            } else if value.as_bytes()[0].is_ascii_digit() {
                // Parse file
                let mut f = value
                    .split(" ")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>();
                let s = f[0].parse::<usize>().unwrap();
                total_size += s;
                current_files.push(FileSystem::File {
                    size: s,
                    name: f.pop().unwrap(),
                });
            } else {
                /*
                Starts with:
                "dir <dirname>"
                "$ ls"
                */
            }
        } else {
            // EOF
            break;
        }
    }

    FileSystem::Folder {
        files: current_files,
        name,
        size: total_size,
    }
}

fn parse_fs(input: reader::Reader) -> FileSystem {
    // Starting point for parsing the structure of file system.
    let mut lines = input.lines();
    lines.reverse();
    lines.pop(); // Always "$ cd /"
    parse_folder(&mut lines, "/".to_string())
}

fn traverse_fs(fs: &FileSystem) -> usize {
    let mut res = 0;

    if let FileSystem::Folder { files, name, size } = fs {
        if size <= &100_000 {
            res += size;
        }

        for file in files.iter() {
            match file {
                FileSystem::Folder { files, name, size } => res += traverse_fs(file),
                _ => {}
            }
        }
    }

    res
}

fn part_one(input: reader::Reader) -> usize {
    traverse_fs(&parse_fs(input))
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
    assert_eq!(part_one(get_test_input()), 95437);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(input()), 1086293);
}

#[test]
fn test_part_two_example() {
    assert_eq!(part_two(get_test_input()), 8381165);
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
