use std::collections::HashSet;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(tail.clone());

    for line in s.split("\n").map(|line| {line.chars().collect::<Vec<char>>()}) {
        let direction = line[0];
        let steps = line[2..line.len()].iter().collect::<String>().parse().unwrap();
        for _step in 0..steps {
            let diagonal = ((head.0 - tail.0) as i32).abs() == 1 && ((head.1 - tail.1) as i32)
                .abs() == 1;
            let old_head = head.clone();
            match direction {
                'L' => { head = (head.0 - 1, head.1) },
                'R' => {head = (head.0 + 1, head.1)},
                'U' => {head = (head.0, head.1 - 1)},
                'D' => {head = (head.0, head.1 + 1)},
                _ => panic!()
            }
            if diagonal && ((head.0 - tail.0) as i32).abs() + ((head.1 - tail.1) as i32)
                .abs() == 3 {
                tail = old_head;
            } else {
                if ((head.0 - tail.0) as i32).abs() == 2 {
                    tail = (tail.0 + (if head.0 > tail.0 {1} else {-1}), tail.1);
                } else if ((head.1 - tail.1) as i32).abs() == 2 {
                    tail = (tail.0, tail.1 + (if head.1 > tail.1 {1} else {-1}));
                }
            }
            visited.insert(tail.clone());
        }
    }

    return visited.len().to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day09/part1_test_output.txt");
    let actual = part1("src/day09/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day09/input.txt");
    io::write_to_file("src/day09/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for line in s.split("\n").map(|line| {line.chars().collect::<Vec<char>>()}) {
        let direction = line[0];
        let steps = line[2..line.len()].iter().collect::<String>().parse().unwrap();
        for _step in 0..steps {
            let mut next_knots = knots.clone();
            let mut new_head = next_knots[0];
            match direction {
                'L' => { new_head = (new_head.0 - 1, new_head.1) },
                'R' => {new_head = (new_head.0 + 1, new_head.1)},
                'U' => {new_head = (new_head.0, new_head.1 - 1)},
                'D' => {new_head = (new_head.0, new_head.1 + 1)},
                _ => panic!()
            }
            next_knots[0] = new_head;
            for (i, old_tail) in knots[1..knots.len()].iter().enumerate() {
                let head = next_knots[i];
                let mut tail = old_tail.clone();
                if ((head.0 - tail.0) as i32).abs() + ((head.1 - tail.1) as i32).abs() >= 3 {
                    tail = (tail.0 + (if head.0 > tail.0 {1} else {-1}), tail.1 + (if head.1 > tail.1 {1} else {-1}));
                } else {
                    if ((head.0 - tail.0) as i32).abs() == 2 {
                        tail = (tail.0 + (if head.0 > tail.0 { 1 } else { -1 }), tail.1);
                    } else if ((head.1 - tail.1) as i32).abs() == 2 {
                        tail = (tail.0, tail.1 + (if head.1 > tail.1 { 1 } else { -1 }));
                    }
                }
                next_knots[i + 1] = tail;
            }
            visited.insert(next_knots[9].clone());
            knots = next_knots.clone();
        }
    }

    return visited.len().to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day09/part2_test_output.txt");
    let actual = part2("src/day09/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day09/input.txt");
    io::write_to_file("src/day09/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
