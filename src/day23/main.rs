use std::collections::{HashMap, HashSet};
use itertools::Itertools;
pub use utils::io;

enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn can_move(&self, elf: &(i32, i32), blocked: &HashSet<(i32, i32)>) -> bool {
        let elf = elf.clone();
        (-1..=1).cartesian_product(-1..=1).any(|e| {
            blocked.contains(&(e.0 + elf.0, e.1 + elf.1)) && e != (0, 0)
        }) && self.neighbors(&elf).iter().all(|e| {
            !blocked.contains(e)
        })
    }

    fn neighbors(&self, elf: &(i32, i32)) -> Vec<(i32, i32)> {
        let new_spot = self.move_elf(elf);
        (-1..=1).map(|i| -> (i32, i32) {
            match self {
                Direction::North | Direction::South => (new_spot.0 + i, new_spot.1),
                Direction::West | Direction::East => (new_spot.0, new_spot.1 + i),
            }
        }).collect_vec()
    }

    fn move_elf(&self, elf: &(i32, i32)) -> (i32, i32) {
        let elf = elf.clone();
        match self {
            Direction::North => (elf.0, elf.1 - 1),
            Direction::South => (elf.0, elf.1 + 1),
            Direction::West => (elf.0 - 1, elf.1),
            Direction::East => (elf.0 + 1, elf.1),
        }
    }
}

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in s.split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let direction_orders = vec![
        vec![Direction::North, Direction::South, Direction::West, Direction::East],
        vec![Direction::South, Direction::West, Direction::East, Direction::North],
        vec![Direction::West, Direction::East, Direction::North, Direction::South],
        vec![Direction::East, Direction::North, Direction::South, Direction::West],
    ];

    for i in 0..10 {
        let direction_order = &direction_orders[i % 4];
        let mut new_to_old: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for elf in elves.iter() {
            let mut new_pos = elf.clone();
            for direction in direction_order.iter() {
                if direction.can_move(elf, &elves) {
                    new_pos = direction.move_elf(elf);
                    break;
                }
            }
            let mut locations = new_to_old.get(&new_pos).unwrap_or(&vec![]).clone();
            locations.push(elf.clone());
            new_to_old.insert(new_pos, locations);
        }
        let mut new_elf_locations: HashSet<(i32, i32)> = HashSet::new();
        for (new_pos, old_poses) in new_to_old.into_iter() {
            if old_poses.len() > 1 {
                new_elf_locations.extend(old_poses.into_iter());
            } else {
                new_elf_locations.insert(new_pos);
            }
        }
        elves = new_elf_locations;
    }


    let min_x = elves.iter().min_by(|&&e1, &&e2| e1.0.partial_cmp(&e2.0).unwrap()).unwrap().clone
    ().0;
    let min_y = elves.iter().min_by(|&&e1, &&e2| e1.1.partial_cmp(&e2.1).unwrap()).unwrap().clone
    ().1;
    let max_x = elves.iter().max_by(|&&e1, &&e2| e1.0.partial_cmp(&e2.0).unwrap()).unwrap().clone
    ().0;
    let max_y = elves.iter().max_by(|&&e1, &&e2| e1.1.partial_cmp(&e2.1).unwrap()).unwrap().clone
    ().1;

    let num_elves = elves.len() as i32;

    return ((max_x - min_x + 1) * (max_y - min_y + 1) - num_elves).to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day23/part1_test_output.txt");
    let actual = part1("src/day23/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day23/input.txt");
    io::write_to_file("src/day23/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in s.split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let direction_orders = vec![
        vec![Direction::North, Direction::South, Direction::West, Direction::East],
        vec![Direction::South, Direction::West, Direction::East, Direction::North],
        vec![Direction::West, Direction::East, Direction::North, Direction::South],
        vec![Direction::East, Direction::North, Direction::South, Direction::West],
    ];

    let mut i = 1;

    loop {
        i += 1;
        let direction_order = &direction_orders[i % 4];
        let mut new_to_old: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for elf in elves.iter() {
            let mut new_pos = elf.clone();
            for direction in direction_order.iter() {
                if direction.can_move(elf, &elves) {
                    new_pos = direction.move_elf(elf);
                    break;
                }
            }
            let mut locations = new_to_old.get(&new_pos).unwrap_or(&vec![]).clone();
            locations.push(elf.clone());
            new_to_old.insert(new_pos, locations);
        }
        let mut new_elf_locations: HashSet<(i32, i32)> = HashSet::new();
        for (new_pos, old_poses) in new_to_old.into_iter() {
            if old_poses.len() > 1 {
                new_elf_locations.extend(old_poses.into_iter());
            } else {
                new_elf_locations.insert(new_pos);
            }
        }
        if elves.symmetric_difference(&new_elf_locations).into_iter().count() == 0 {
            break;
        }
        elves = new_elf_locations;
    }

    return i.to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day23/part2_test_output.txt");
    let actual = part2("src/day23/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day23/input.txt");
    io::write_to_file("src/day23/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
