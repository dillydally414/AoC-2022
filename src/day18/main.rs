use std::cmp::{min, Ordering};
use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut squares: HashSet<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32), (i32, i32, i32))> = HashSet::new();
    let mut duplicate_squares: HashSet<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32), (i32, i32, i32))> = HashSet::new();

    for line in s.split("\n") {
        let mut coords = line.split(",").map(|num| num.parse::<i32>().unwrap());
        let top_left_front = (coords.next().unwrap(), coords.next().unwrap(), coords.next()
            .unwrap());
        let top_left_back = (top_left_front.0, top_left_front.1, top_left_front.2 + 1);
        let top_right_front = (top_left_front.0 + 1, top_left_front.1, top_left_front.2);
        let top_right_back = (top_left_front.0 + 1, top_left_front.1, top_left_front.2 + 1);
        let bot_left_front = (top_left_front.0, top_left_front.1 + 1, top_left_front.2);
        let bot_left_back = (top_left_front.0, top_left_front.1 + 1, top_left_front.2 + 1);
        let bot_right_front = (top_left_front.0 + 1, top_left_front.1 + 1, top_left_front.2);
        let bot_right_back = (top_left_front.0 + 1, top_left_front.1 + 1, top_left_front.2 + 1);
        let new_squares = vec![
            (top_left_front, top_left_back, top_right_front, top_right_back),
            (top_left_front, top_left_back, bot_left_front, bot_left_back),
            (top_left_front, top_right_front, bot_left_front, bot_right_front),
            (top_right_front, top_right_back, bot_right_front, bot_right_back),
            (bot_left_front, bot_left_back, bot_right_front, bot_right_back),
            (top_left_back, top_right_back, bot_left_back, bot_right_back),
        ];
        for s in new_squares {
            let new_square = squares.insert(s);
            if !new_square {
                duplicate_squares.insert(s);
            }
        }
    };

    return (squares.len() - duplicate_squares.len()).to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day18/part1_test_output.txt");
    let actual = part1("src/day18/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day18/input.txt");
    io::write_to_file("src/day18/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut squares: HashSet<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32), (i32, i32, i32))> = HashSet::new();
    let mut duplicate_squares: HashSet<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32), (i32, i32, i32))> = HashSet::new();

    for line in s.split("\n") {
        let mut coords = line.split(",").map(|num| num.parse::<i32>().unwrap());
        let top_left_front = (coords.next().unwrap(), coords.next().unwrap(), coords.next()
            .unwrap());
        let top_left_back = (top_left_front.0, top_left_front.1, top_left_front.2 + 1);
        let top_right_front = (top_left_front.0 + 1, top_left_front.1, top_left_front.2);
        let top_right_back = (top_left_front.0 + 1, top_left_front.1, top_left_front.2 + 1);
        let bot_left_front = (top_left_front.0, top_left_front.1 + 1, top_left_front.2);
        let bot_left_back = (top_left_front.0, top_left_front.1 + 1, top_left_front.2 + 1);
        let bot_right_front = (top_left_front.0 + 1, top_left_front.1 + 1, top_left_front.2);
        let bot_right_back = (top_left_front.0 + 1, top_left_front.1 + 1, top_left_front.2 + 1);
        cubes.insert(top_left_front);
        let new_squares = vec![
            (top_left_front, top_left_back, top_right_front, top_right_back),
            (top_left_front, top_left_back, bot_left_front, bot_left_back),
            (top_left_front, top_right_front, bot_left_front, bot_right_front),
            (top_right_front, top_right_back, bot_right_front, bot_right_back),
            (bot_left_front, bot_left_back, bot_right_front, bot_right_back),
            (top_left_back, top_right_back, bot_left_back, bot_right_back),
        ];
        for s in new_squares {
            let new_square = squares.insert(s);
            if !new_square {
                duplicate_squares.insert(s);
            }
        }
    };

    let mut pocket_cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut flood_cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    let min_x = cubes.iter().min_by(|coord1, coord2| coord1.0.partial_cmp(&coord2.0).unwrap())
        .unwrap().0 - 1;
    let min_y = cubes.iter().min_by(|coord1, coord2| coord1.1.partial_cmp(&coord2.1).unwrap())
        .unwrap().1 - 1;
    let min_z = cubes.iter().min_by(|coord1, coord2| coord1.2.partial_cmp(&coord2.2).unwrap())
        .unwrap().2 - 1;
    let max_x = cubes.iter().max_by(|coord1, coord2| coord1.0.partial_cmp(&coord2.0).unwrap())
        .unwrap().0 + 1;
    let max_y = cubes.iter().max_by(|coord1, coord2| coord1.1.partial_cmp(&coord2.1).unwrap())
        .unwrap().1 + 1;
    let max_z = cubes.iter().max_by(|coord1, coord2| coord1.2.partial_cmp(&coord2.2).unwrap())
        .unwrap().2 + 1;

    let mut flood_queue = vec![(min_x, min_y, min_z)];

    while let Some(next_cube) = flood_queue.pop() {
        if cubes.contains(&next_cube) || flood_cubes.contains(&next_cube) {
            continue;
        }
        flood_cubes.insert(next_cube.clone());
        let neighbors = vec![
            (next_cube.0 - 1, next_cube.1, next_cube.2),
            (next_cube.0 + 1, next_cube.1, next_cube.2),
            (next_cube.0, next_cube.1 - 1, next_cube.2),
            (next_cube.0, next_cube.1 + 1, next_cube.2),
            (next_cube.0, next_cube.1, next_cube.2 - 1),
            (next_cube.0, next_cube.1, next_cube.2 + 1),
        ].into_iter().filter(|(x, y, z)| x >= &min_x && x <= &max_x
            && y >= &min_y && y <= &max_y && z >= &min_z && z <= &max_z);
        flood_queue.extend(neighbors);
    }

    for i in min_x..=max_x {
        for j in min_y..=max_y {
            for k in min_z..=max_z {
                let mut pocket: HashSet<(i32, i32, i32)> = HashSet::new();
                let mut queue: Vec<(i32, i32, i32)> = vec![(i, j, k)];
                while let Some(curr_cube) = queue.pop() {
                    if pocket_cubes.contains(&curr_cube) || cubes.contains(&curr_cube) {
                        continue;
                    }
                    pocket.insert(curr_cube);
                    let neighbors = (-1..=1).map(|inc| {
                        vec![(curr_cube.0 + inc, curr_cube.1, curr_cube.2),
                             (curr_cube.0, curr_cube.1 + inc, curr_cube.2),
                             (curr_cube.0, curr_cube.1, curr_cube.2 + inc)]
                    }).flatten().filter(|coord| !pocket.contains(coord) && !queue.contains(coord) &&
                            !cubes.contains(coord)).collect::<Vec<(i32, i32, i32)>>();
                    if neighbors.iter().filter(|(x, y, z)| x >= &min_x && x <= &max_x
                        && y >= &min_y && y <= &max_y && z >= &min_z && z <= &max_z).count() > 0 {
                        queue.clear();
                        pocket.clear();
                        break;
                    }
                    queue.extend(neighbors);
                }
                pocket_cubes.extend(pocket);
            }
        }
    }

    for i in min_x..=max_x {
        for j in min_y..=max_y {
            for k in min_z..=max_z {
                let cube = (i, j, k);
                if !flood_cubes.contains(&cube) && !cubes.contains(&cube) {
                    pocket_cubes.insert(cube);
                }
            }
        }
    }

    for cube in pocket_cubes.iter() {
        let top_left_front = cube.clone();
        let top_left_back = (top_left_front.0, top_left_front.1, top_left_front.2 + 1);
        let top_right_front = (top_left_front.0 + 1, top_left_front.1, top_left_front.2);
        let top_right_back = (top_left_front.0 + 1, top_left_front.1, top_left_front.2 + 1);
        let bot_left_front = (top_left_front.0, top_left_front.1 + 1, top_left_front.2);
        let bot_left_back = (top_left_front.0, top_left_front.1 + 1, top_left_front.2 + 1);
        let bot_right_front = (top_left_front.0 + 1, top_left_front.1 + 1, top_left_front.2);
        let bot_right_back = (top_left_front.0 + 1, top_left_front.1 + 1, top_left_front.2 + 1);
        let new_squares = vec![
            (top_left_front, top_left_back, top_right_front, top_right_back),
            (top_left_front, top_left_back, bot_left_front, bot_left_back),
            (top_left_front, top_right_front, bot_left_front, bot_right_front),
            (top_right_front, top_right_back, bot_right_front, bot_right_back),
            (bot_left_front, bot_left_back, bot_right_front, bot_right_back),
            (top_left_back, top_right_back, bot_left_back, bot_right_back),
        ];
        for s in new_squares {
            let new_square = squares.insert(s);
            if !new_square {
                duplicate_squares.insert(s);
            }
        }
    }

    return (squares.len() - duplicate_squares.len()).to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day18/part2_test_output.txt");
    let actual = part2("src/day18/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day18/input.txt");
    io::write_to_file("src/day18/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
