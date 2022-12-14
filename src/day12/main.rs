use std::collections::HashMap;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut elevations: HashMap<(usize, usize), i8> = HashMap::new();
    let mut curr: (usize, usize) = (1, 1);
    let mut end: (usize, usize) = (1, 1);

    for (row_idx, row) in s.split("\n").enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char == 'S' {
                curr = (row_idx + 1, col_idx + 1);
                elevations.insert((row_idx + 1, col_idx + 1), 0);
            } else if char == 'E' {
                end = (row_idx + 1, col_idx + 1);
                elevations.insert((row_idx + 1, col_idx + 1), ('z'.to_digit(36).unwrap() - 'a'
                    .to_digit(36).unwrap()) as i8);
            } else {
                elevations.insert((row_idx + 1, col_idx + 1), (char.to_digit(36).unwrap() - 'a'
                    .to_digit(36).unwrap()) as i8);
            }
        }
    }

    let mut edges: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for (&location, &elevation) in elevations.iter() {
        let neighbors = vec![(location.0 - 1, location.1), (location.0 + 1, location.1),
            (location.0, location.1 - 1), (location.0, location.1 + 1)].into_iter().filter
        (|&neighbor_loc| {
            elevations.contains_key(&neighbor_loc) && elevations.get(&neighbor_loc).unwrap() -
                elevation <= 1
        });
        edges.insert(location, neighbors.collect());
    }

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    distances.insert(curr, 0);

    let mut queue: Vec<&(usize, usize)> = edges.keys().collect();

    while !queue.is_empty() {
        let (next_idx, &next) = queue.iter().enumerate().min_by(|&(idx1, &location1), &(idx2,
            &location2)| {
            distances.get(location1).unwrap_or(&100000).partial_cmp(distances.get(location2)
                .unwrap_or(&100000)).unwrap()
        }).unwrap();
        if next == &end {
            break;
        }

        for &v in edges.get(next).unwrap_or(&vec![]) {
            if queue.contains(&&v) {
                let new_dist = distances.get(next).unwrap() + 1;
                if &new_dist < distances.get(&v).unwrap_or(&1000000) {
                    distances.insert(v, new_dist);
                }
            }
        }

        queue.remove(next_idx);
    }

    return distances.get(&end).unwrap().to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day12/part1_test_output.txt");
    let actual = part1("src/day12/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day12/input.txt");
    io::write_to_file("src/day12/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut elevations: HashMap<(usize, usize), i8> = HashMap::new();
    let mut starts: Vec<(usize, usize)> = vec![];
    let mut end: (usize, usize) = (1, 1);

    for (row_idx, row) in s.split("\n").enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char == 'S' || char == 'a' {
                starts.push((row_idx + 1, col_idx + 1));
                elevations.insert((row_idx + 1, col_idx + 1), 0);
            } else if char == 'E' {
                end = (row_idx + 1, col_idx + 1);
                elevations.insert((row_idx + 1, col_idx + 1), ('z'.to_digit(36).unwrap() - 'a'
                    .to_digit(36).unwrap()) as i8);
            } else {
                elevations.insert((row_idx + 1, col_idx + 1), (char.to_digit(36).unwrap() - 'a'
                    .to_digit(36).unwrap()) as i8);
            }
        }
    }

    let mut edges: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for (&location, &elevation) in elevations.iter() {
        let neighbors = vec![(location.0 - 1, location.1), (location.0 + 1, location.1),
                             (location.0, location.1 - 1), (location.0, location.1 + 1)].into_iter().filter
        (|&neighbor_loc| {
            elevations.contains_key(&neighbor_loc) && elevations.get(&neighbor_loc).unwrap() -
                elevation <= 1
        });
        edges.insert(location, neighbors.collect());
    }

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    for a in starts {
        distances.insert(a, 0);
    }

    let mut queue: Vec<&(usize, usize)> = edges.keys().collect();

    while !queue.is_empty() {
        let (next_idx, &next) = queue.iter().enumerate().min_by(|&(idx1, &location1), &(idx2,
            &location2)| {
            distances.get(location1).unwrap_or(&100000).partial_cmp(distances.get(location2)
                .unwrap_or(&100000)).unwrap()
        }).unwrap();
        if next == &end {
            break;
        }

        for &v in edges.get(next).unwrap_or(&vec![]) {
            if queue.contains(&&v) {
                let new_dist = distances.get(next).unwrap() + 1;
                if &new_dist < distances.get(&v).unwrap_or(&1000000) {
                    distances.insert(v, new_dist);
                }
            }
        }

        queue.remove(next_idx);
    }

    return distances.get(&end).unwrap().to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day12/part2_test_output.txt");
    let actual = part2("src/day12/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day12/input.txt");
    io::write_to_file("src/day12/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
