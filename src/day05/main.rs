pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let input_split: Vec<&str> = s.split("\n\n").collect();
    let crates_str = input_split[0];
    let instructions_str = input_split[1];

    let crates_num: Vec<i32> = crates_str.split('\n').last().unwrap().split_whitespace().map
    (|str| str.parse().unwrap()).collect();

    let mut crates: Vec<Vec<char>> = vec![vec![]; crates_num.len()];

    for line in crates_str.split('\n') {
        for crate_idx in &crates_num {
            let char = line.chars().nth((crate_idx * 4 - 3) as usize).unwrap_or(' ');
            if char.is_alphabetic() {
                crates[(crate_idx - 1) as usize].insert(0, char);
            }
        }
    }

    for line in instructions_str.split('\n') {
        let instr: Vec<i32> = line.split(char::is_alphabetic).filter_map(|num| num.trim()
            .parse().ok()).collect();
        for _i in 0..instr[0] {
            let moving_crate = crates[(instr[1] - 1) as usize].pop().unwrap_or(' ');
            if moving_crate.is_whitespace() {
                break;
            } else {
                crates[(instr[2] - 1) as usize].push(moving_crate);
            }
        }
    };

    return crates.into_iter().map(|mut stack| stack.pop().unwrap()).collect();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day05/part1_test_output.txt");
    let actual = part1("src/day05/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day05/input.txt");
    io::write_to_file("src/day05/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let input_split: Vec<&str> = s.split("\n\n").collect();
    let crates_str = input_split[0];
    let instructions_str = input_split[1];

    let crates_num: Vec<i32> = crates_str.split('\n').last().unwrap().split_whitespace().map
    (|str| str.parse().unwrap()).collect();

    let mut crates: Vec<Vec<char>> = vec![vec![]; crates_num.len()];

    for line in crates_str.split('\n') {
        for crate_idx in &crates_num {
            let char = line.chars().nth((crate_idx * 4 - 3) as usize).unwrap_or(' ');
            if char.is_alphabetic() {
                crates[(crate_idx - 1) as usize].insert(0, char);
            }
        }
    }

    for line in instructions_str.split('\n') {
        let instr: Vec<i32> = line.split(char::is_alphabetic).filter_map(|num| num.trim()
            .parse().ok()).collect();
        let mut temp_vec = vec![];
        for _i in 0..instr[0] {
            let moving_crate = crates[(instr[1] - 1) as usize].pop().unwrap_or(' ');
            if moving_crate.is_whitespace() {
                break;
            } else {
                temp_vec.push(moving_crate);
            }
        }
        temp_vec.reverse();
        for value in temp_vec {
            crates[(instr[2] - 1) as usize].push(value);
        }
    };

    return crates.into_iter().map(|mut stack| stack.pop().unwrap()).collect();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day05/part2_test_output.txt");
    let actual = part2("src/day05/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day05/input.txt");
    io::write_to_file("src/day05/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
