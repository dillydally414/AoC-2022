pub use utils::io;

fn part1(filename: &str) -> String {
    let s = io::file_to_str(filename);

    let mut max = 0;
    let mut curr = 0;

    for line in s.split('\n') {
        if line.is_empty() {
            max = max.max(curr);
            curr = 0;
        } else {
            curr += line.parse::<i32>().unwrap();
        }
    }

    return max.max(curr).to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day01/part1_test_output.txt");
    let actual = part1("src/day01/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day01/input.txt");
    io::write_to_file("src/day01/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s = io::file_to_str(filename);

    let mut max = Vec::from([0, 0, 0]);
    let mut curr = 0;

    for line in s.split('\n') {
        if line.is_empty() {
            match curr {
                c if c > max[0] => {max[2] = max[1]; max[1] = max[0]; max[0] = c;}
                c if c > max[1] => {max[2] = max[1]; max[1] = c;}
                c if c > max[2] => {max[2] = c;}
                _ => {}
            }
            curr = 0;
        } else {
            curr += line.parse::<i32>().unwrap();
        }
    }

    match curr {
        c if c > max[0] => {max[2] = max[1]; max[1] = max[0]; max[0] = c;}
        c if c > max[1] => {max[2] = max[1]; max[1] = c;}
        c if c > max[2] => {max[2] = c;}
        _ => {}
    }

    return max.iter().sum::<i32>().to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day01/part2_test_output.txt");
    let actual = part2("src/day01/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day01/input.txt");
    io::write_to_file("src/day01/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
