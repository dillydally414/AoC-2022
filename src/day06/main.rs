use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut marker: Vec<char> = vec![];

    for (i, c) in s.chars().enumerate() {
        if marker.len() < 4 {
            marker.push(c);
        } else {
            marker.remove(0);
            marker.push(c);
        }
        if HashSet::<&char>::from_iter(marker.iter()).len() == 4 {
            return (i + 1).to_string();
        }
    }

    return s.len().to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day06/part1_test_output.txt");
    let actual = part1("src/day06/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day06/input.txt");
    io::write_to_file("src/day06/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut marker: Vec<char> = vec![];

    for (i, c) in s.chars().enumerate() {
        if marker.len() < 14 {
            marker.push(c);
        } else {
            marker.remove(0);
            marker.push(c);
        }
        if HashSet::<&char>::from_iter(marker.iter()).len() == 14 {
            return (i + 1).to_string();
        }
    }

    return s.len().to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day06/part2_test_output.txt");
    let actual = part2("src/day06/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day06/input.txt");
    io::write_to_file("src/day06/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
