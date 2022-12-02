pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let tot_points: i32 = s.split('\n').into_iter().map(|line| -> (char, char) {
        return (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
    }).map(|(p1, p2)| {
        let choice_points: i32 = match p2 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Unexpected char for player 2: {}", p2)
        };
        let outcome_points: i32 = match p1 {
            'A' => (choice_points % 3) * 3,
            'B' => ((choice_points - 1) % 3) * 3,
            'C' => ((choice_points + 1) % 3) * 3,
            _ => panic!("Unexpected char for player 1: {}", p1)
        };
        return choice_points + outcome_points;
    }).sum();

    return tot_points.to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day2/part1_test_output.txt");
    let actual = part1("src/day2/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day2/input.txt");
    io::write_to_file("src/day2/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let tot_points: i32 = s.split('\n').into_iter().map(|line| -> (char, char) {
        return (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
    }).map(|(p1, p2)| {
        let outcome_points: i32 = match p2 {
            'X' => 0, // 2
            'Y' => 3, // 3
            'Z' => 6, // 1
            _ => panic!("Unexpected char for player 2: {}", p2)
        };
        let mut choice_points: i32 = match p1 {
            'A' => outcome_points / 3,
            'B' => ((outcome_points / 3) + 1),
            'C' => ((outcome_points / 3) + 2) % 3,
            _ => panic!("Unexpected char for player 1: {}", p1)
        };
        if choice_points == 0 {
            choice_points = 3;
        }
        return choice_points + outcome_points;
    }).sum();

    return tot_points.to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day2/part2_test_output.txt");
    let actual = part2("src/day2/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day2/input.txt");
    io::write_to_file("src/day2/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
