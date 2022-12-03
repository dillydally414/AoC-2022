use std::collections::HashSet;
use std::iter::FromIterator;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let tot_priorities: u32 = s.split('\n').into_iter().map(|line| -> (&str, &str) {
        return line.split_at(line.len() / 2);
    }).map(|(ruck1, ruck2)| {
        let ruck1_contents: HashSet<char> = HashSet::from_iter(ruck1.chars());
        let ruck2_contents: HashSet<char> = HashSet::from_iter(ruck2.chars());
        let overlap: &char = ruck1_contents.intersection(&ruck2_contents).nth(0).unwrap();
        let priority = if overlap.is_uppercase() {
            *overlap as u32 - 'A' as u32 + 27
        } else {
            *overlap as u32 - 'a' as u32 + 1
        };
        return priority;
    }).sum();

    return tot_priorities.to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day03/part1_test_output.txt");
    let actual = part1("src/day03/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day03/input.txt");
    io::write_to_file("src/day03/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let ruck_iter = s.split('\n').into_iter().map(|line| -> HashSet<char> {
        return HashSet::from_iter(line.chars());
    });

    let mut tot_priorities = 0;
    let mut curr_overlap: HashSet<char> = HashSet::new();

    for (i, ruck) in ruck_iter.enumerate() {
        if i % 3 == 0 {
            curr_overlap = ruck;
        } else {
            curr_overlap = HashSet::from_iter(curr_overlap.intersection(&ruck).cloned().into_iter
            ());
        }
        if i % 3 == 2 {
            let overlap: char = curr_overlap.drain().into_iter().nth(0).unwrap();
            let priority = if overlap.is_uppercase() {
                overlap as u32 - 'A' as u32 + 27
            } else {
                overlap as u32 - 'a' as u32 + 1
            };
            tot_priorities += priority;
        }
    }

    return tot_priorities.to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day03/part2_test_output.txt");
    let actual = part2("src/day03/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day03/input.txt");
    io::write_to_file("src/day03/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
