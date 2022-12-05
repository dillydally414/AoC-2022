use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Range;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    return s;
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/dayXX/part1_test_output.txt");
    let actual = part1("src/dayXX/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/dayXX/input.txt");
    io::write_to_file("src/dayXX/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    return s;
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/dayXX/part2_test_output.txt");
    let actual = part2("src/dayXX/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/dayXX/input.txt");
    io::write_to_file("src/dayXX/part2_output.txt", actual);
}

fn main() {
    run_part1();
}
