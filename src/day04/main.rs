use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Range;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let overlaps: i32 = s.split('\n').map(|line| -> (HashSet<i32>, HashSet<i32>) {
        let ranges: Vec<&str> = line.split(',').collect();
        let range1: Vec<i32> = ranges[0].split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        let range2: Vec<i32> = ranges[1].split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        return (HashSet::from_iter((range1[0]..range1[1] + 1)), HashSet::from_iter(
            (range2[0]..range2[1] + 1)))
    }).fold(0, |i, (set1, set2)| {
        if set1.is_superset(&set2) || set2.is_superset(&set1) {
            i + 1
        } else {
            i
        }
    });

    return overlaps.to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day04/part1_test_output.txt");
    let actual = part1("src/day04/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day04/input.txt");
    io::write_to_file("src/day04/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let overlaps: i32 = s.split('\n').map(|line| -> (HashSet<i32>, HashSet<i32>) {
        let ranges: Vec<&str> = line.split(',').collect();
        let range1: Vec<i32> = ranges[0].split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        let range2: Vec<i32> = ranges[1].split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        return (HashSet::from_iter((range1[0]..range1[1] + 1)), HashSet::from_iter(
            (range2[0]..range2[1] + 1)))
    }).fold(0, |i, (set1, set2)| {
        if set1.intersection(&set2).collect::<Vec<&i32>>().is_empty() {
            i
        } else {
            i + 1
        }
    });

    return overlaps.to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day04/part2_test_output.txt");
    let actual = part2("src/day04/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day04/input.txt");
    io::write_to_file("src/day04/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
