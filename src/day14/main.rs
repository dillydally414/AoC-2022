use std::collections::HashSet;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut rocks: HashSet<(usize, usize)> = HashSet::new();

    for line in s.split("\n") {
        let mut paths = line.split(" -> ").map(|point| point.split_once(",")
            .unwrap()).map(|(x, y)| -> (usize, usize) {(x.parse().unwrap(), y.parse().unwrap())})
            .into_iter();

        let mut prev_path = paths.next().unwrap();
        while let Some(next_path) = paths.next() {
            for i in prev_path.0.min(next_path.0)..=prev_path.0.max(next_path.0) {
                for j in prev_path.1.min(next_path.1)..=prev_path.1.max(next_path.1) {
                    rocks.insert((i, j));
                }
            }
            prev_path = next_path;
        }
    }

    let bottom_rock = rocks.iter().max_by(|&rock1, &rock2| rock1.1.partial_cmp(&rock2.1).unwrap()
    ).unwrap();

    let mut sand: HashSet<(usize, usize)> = HashSet::new();

    let mut sand_drop: (usize, usize) = (500, 0);

    loop {
        let bottom_left = (sand_drop.0 - 1, sand_drop.1 + 1);
        let bottom = (sand_drop.0 , sand_drop.1 + 1);
        let bottom_right = (sand_drop.0 + 1, sand_drop.1 + 1);

        if rocks.contains(&bottom) || sand.contains(&bottom) {
            if rocks.contains(&bottom_left) || sand.contains(&bottom_left) {
                if rocks.contains(&bottom_right) || sand.contains(&bottom_right) {
                    sand.insert(sand_drop.clone());
                    sand_drop = (500, 0);
                } else {
                    sand_drop = bottom_right;
                }
            } else {
                sand_drop = bottom_left;
            }
        } else {
            sand_drop = bottom;
        }

        if sand_drop.1 >= bottom_rock.1 {
            break;
        }
    }

    return sand.len().to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day14/part1_test_output.txt");
    let actual = part1("src/day14/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day14/input.txt");
    io::write_to_file("src/day14/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut rocks: HashSet<(usize, usize)> = HashSet::new();

    for line in s.split("\n") {
        let mut paths = line.split(" -> ").map(|point| point.split_once(",")
            .unwrap()).map(|(x, y)| -> (usize, usize) {(x.parse().unwrap(), y.parse().unwrap())})
            .into_iter();

        let mut prev_path = paths.next().unwrap();
        while let Some(next_path) = paths.next() {
            for i in prev_path.0.min(next_path.0)..=prev_path.0.max(next_path.0) {
                for j in prev_path.1.min(next_path.1)..=prev_path.1.max(next_path.1) {
                    rocks.insert((i, j));
                }
            }
            prev_path = next_path;
        }
    }

    let bottom_rock = rocks.iter().max_by(|&rock1, &rock2| rock1.1.partial_cmp(&rock2.1).unwrap()
    ).unwrap();

    let floor = bottom_rock.1 + 2;

    let left_rock = rocks.iter().min_by(|&rock1, &rock2| rock1.0.partial_cmp(&rock2.0).unwrap()
    ).unwrap();
    let right_rock = rocks.iter().max_by(|&rock1, &rock2| rock1.0.partial_cmp(&rock2.0).unwrap()
    ).unwrap();

    for i in 0..=right_rock.0 + 500 {
        rocks.insert((i, floor));
    }

    let mut sand: HashSet<(usize, usize)> = HashSet::new();

    let mut sand_drop: (usize, usize) = (500, 0);

    while !sand.contains(&(500, 0)) {
        let bottom_left = (sand_drop.0 - 1, sand_drop.1 + 1);
        let bottom = (sand_drop.0 , sand_drop.1 + 1);
        let bottom_right = (sand_drop.0 + 1, sand_drop.1 + 1);

        if rocks.contains(&bottom) || sand.contains(&bottom) {
            if rocks.contains(&bottom_left) || sand.contains(&bottom_left) {
                if rocks.contains(&bottom_right) || sand.contains(&bottom_right) {
                    sand.insert(sand_drop.clone());
                    sand_drop = (500, 0);
                } else {
                    sand_drop = bottom_right;
                }
            } else {
                sand_drop = bottom_left;
            }
        } else {
            sand_drop = bottom;
        }
    }

    return sand.len().to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day14/part2_test_output.txt");
    let actual = part2("src/day14/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day14/input.txt");
    io::write_to_file("src/day14/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
