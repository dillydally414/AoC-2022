use std::collections::{HashMap, HashSet};
pub use utils::io;

fn calc_size(dir: &String, dirsizes: &mut HashMap<String, i32>, filesystem:
&HashMap<String, HashSet<String>>, filesizes: &HashMap<String, i32>) -> i32 {
    if dirsizes.contains_key(dir) {
        return *dirsizes.get(dir).unwrap();
    } else {
        let mut size = 0;
        for child in filesystem.get(dir).unwrap() {
            if filesizes.contains_key(child) {
                size += filesizes.get(child).unwrap();
            } else {
                calc_size(child, dirsizes, filesystem, filesizes);
                size += dirsizes.get(child).unwrap();
            }
        }
        dirsizes.insert(dir.to_string(), size);
        return size;
    }
}

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut filesystem: HashMap<String, HashSet<String>> = HashMap::new();
    let mut filesizes: HashMap<String, i32> = HashMap::new();
    let mut curr_dir = String::new();

    for command in s.split('$').filter(|l| !l.is_empty()) {
        let lines = command.trim().split('\n').collect::<Vec<&str>>();
        let command = lines[0];
        match command.split_whitespace().nth(0).unwrap() {
            "cd" => {
                match command.split_whitespace().nth(1).unwrap() {
                    "/" => curr_dir = "".to_string(),
                    ".." => curr_dir = curr_dir.rsplit_once('/').unwrap().0.to_string(),
                    new_dir => curr_dir = format!("{}/{}", curr_dir, new_dir)
                }
            },
            "ls" => {
                let mut new_set: HashSet<String> = HashSet::new();
                for line in lines {
                    if line == command {continue;}
                    let chunks = line.trim().split_whitespace().collect::<Vec<&str>>();
                    new_set.insert(format!("{}/{}", curr_dir, chunks[1]));
                    if chunks[0] != "dir" {
                        filesizes.insert(format!("{}/{}", curr_dir, chunks[1]), chunks[0].parse().unwrap());
                    }
                }
                if let Some(existing_set) = filesystem.get_mut(&curr_dir) {
                    *existing_set = new_set.union(existing_set).map(|string| string.to_string())
                        .collect();
                } else {
                    filesystem.insert(curr_dir.to_string(), new_set);
                }
            },
            _ => panic!()
        }
    }

    let mut dirsizes: HashMap<String, i32> = HashMap::new();

    for dir in filesystem.keys() {
        let size = calc_size(dir, &mut dirsizes, &filesystem, &filesizes);
        dirsizes.insert(dir.to_string(), size);
    }

    return dirsizes.values().filter(|val| val < &&100000).sum::<i32>().to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day07/part1_test_output.txt");
    let actual = part1("src/day07/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day07/input.txt");
    io::write_to_file("src/day07/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut filesystem: HashMap<String, HashSet<String>> = HashMap::new();
    let mut filesizes: HashMap<String, i32> = HashMap::new();
    let mut curr_dir = String::new();

    for command in s.split('$').filter(|l| !l.is_empty()) {
        let lines = command.trim().split('\n').collect::<Vec<&str>>();
        let command = lines[0];
        match command.split_whitespace().nth(0).unwrap() {
            "cd" => {
                match command.split_whitespace().nth(1).unwrap() {
                    "/" => curr_dir = "".to_string(),
                    ".." => curr_dir = curr_dir.rsplit_once('/').unwrap().0.to_string(),
                    new_dir => curr_dir = format!("{}/{}", curr_dir, new_dir)
                }
            },
            "ls" => {
                let mut new_set: HashSet<String> = HashSet::new();
                for line in lines {
                    if line == command {continue;}
                    let chunks = line.trim().split_whitespace().collect::<Vec<&str>>();
                    new_set.insert(format!("{}/{}", curr_dir, chunks[1]));
                    if chunks[0] != "dir" {
                        filesizes.insert(format!("{}/{}", curr_dir, chunks[1]), chunks[0].parse().unwrap());
                    }
                }
                if let Some(existing_set) = filesystem.get_mut(&curr_dir) {
                    *existing_set = new_set.union(existing_set).map(|string| string.to_string())
                        .collect();
                } else {
                    filesystem.insert(curr_dir.to_string(), new_set);
                }
            },
            _ => panic!()
        }
    }

    let mut dirsizes: HashMap<String, i32> = HashMap::new();

    for dir in filesystem.keys() {
        let size = calc_size(dir, &mut dirsizes, &filesystem, &filesizes);
        dirsizes.insert(dir.to_string(), size);
    }

    let tot_size = dirsizes.get("").unwrap();

    let delete_size = 30000000 - (70000000 - tot_size);

    let mut all_sizes = dirsizes.into_iter().collect::<Vec<(String, i32)>>();
    all_sizes.sort_by(|(dir, size), (dir2, size2)| size.partial_cmp(size2).unwrap());

    return all_sizes.into_iter().filter(|(dir, size)| size > &delete_size).nth(0).unwrap().1.to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day07/part2_test_output.txt");
    let actual = part2("src/day07/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day07/input.txt");
    io::write_to_file("src/day07/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
