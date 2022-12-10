pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut x = 1;
    let mut cycle = 1;

    let mut signal_strengths = 0;

    for line in s.split("\n") {
        if (cycle + 20) % 40 == 0 {
            signal_strengths += x * cycle;
        }
        cycle += 1;
        if line != "noop" {
            let inc = line.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            if (cycle + 20) % 40 == 0 {
                signal_strengths += x * cycle;
            }
            x += inc;
            cycle += 1;
        }
    }

    return signal_strengths.to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day10/part1_test_output.txt");
    let actual = part1("src/day10/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day10/input.txt");
    io::write_to_file("src/day10/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut x: i32 = 1;
    let mut cycle: usize = 1;

    let mut sprite_positions = vec![vec!['.'; 40]; 6];

    for line in s.split("\n") {
        if (x - ((cycle - 1) % 40) as i32).abs() <= 1 {
            sprite_positions[(cycle - 1) / 40][(cycle - 1) % 40] = '#';
        }
        cycle += 1;
        if line != "noop" {
            let inc = line.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            if (x - ((cycle - 1) % 40) as i32).abs() <= 1 {
                sprite_positions[(cycle - 1) / 40][(cycle - 1) % 40] = '#';
            }
            x += inc;
            cycle += 1;
        }
    }

    return sprite_positions.into_iter().map(|line| -> String {line.iter().collect()})
        .collect::<Vec<String>>().join("\n");
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day10/part2_test_output.txt");
    let actual = part2("src/day10/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day10/input.txt");
    io::write_to_file("src/day10/part2_output.txt",actual.replace(".", " "));
}

fn main() {
    run_part2();
}
