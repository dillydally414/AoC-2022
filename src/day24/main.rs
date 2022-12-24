use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use itertools::Itertools;
pub use utils::io;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!()
        }
    }

    fn step(&self, position: (i32, i32), x_borders: (i32, i32), y_borders: (i32, i32)) -> (i32,
                                                                                           i32) {
        let position = self.move_loc(position);
        let position = (
            (position.0 + x_borders.1 - x_borders.0) % (x_borders.1) + x_borders.0,
            (position.1 + y_borders.1 - y_borders.0) % (y_borders.1) + y_borders.0
        );
        position
    }

    fn move_loc(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
        }
    }
}

fn optimal_moves(curr_loc: &(i32, i32), goal: (i32, i32), blizzards: &HashMap<i32, HashMap<(i32,
                                                                                           i32),
    HashSet<Direction>>>, walls: &HashSet<(i32, i32)>, step: i32, cache: &mut HashMap<(i32, i32,
                                                                                     i32), i32>,
                 curr_best: i32,
                 x_bounds: &(i32, i32), y_bounds: &(i32, i32)) -> i32 {
    let curr_loc = curr_loc.clone();
    if cache.contains_key(&(curr_loc.0, curr_loc.1, step)) {
        return cache.get(&(curr_loc.0, curr_loc.1, step)).unwrap().clone();
    } else if step >= curr_best {
        return curr_best;
    } else if curr_loc == goal {
        cache.insert((curr_loc.0, curr_loc.1, step), step);
        return step;
    }

    let new_blizzards = blizzards.get(&step).unwrap();

    let mut options = vec![Direction::Down, Direction::Right, Direction::Up, Direction::Left].into_iter().map
    (|d|
     -> (i32, i32) {d.move_loc(curr_loc)}).collect_vec();
    options.insert(2, curr_loc.clone());

    let mut best = curr_best;
    for o in options.into_iter().filter(|o| !new_blizzards.contains_key(o) && !walls
        .contains(o)) {
        let o_res = optimal_moves(&o, goal.clone(), &blizzards, walls, step + 1,
                                  cache, best, x_bounds, y_bounds);
        best = o_res.min(best);
    };

    cache.insert((curr_loc.0, curr_loc.1, step), best);

    return best;
}

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut prev_blizzards: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
    let mut walls: HashSet<(i32, i32)> = HashSet::new();

    let height = s.split("\n").count() as i32;
    let y_bounds = (1, height - 2);
    let width = s.split("\n").collect::<Vec<&str>>()[0].len() as i32;
    let x_bounds = (1, width - 2);

    for (y, line) in s.split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                walls.insert((x as i32, y as i32));
            } else if char != '.' {
                prev_blizzards.insert((x as i32, y as i32), HashSet::from_iter(vec![Direction::new
                    (char)]
                    .into_iter()));
            }
        }
    }

    let curr_location = (x_bounds.0, y_bounds.0 - 1);
    walls.insert((x_bounds.0, y_bounds.0 - 2));
    let goal = (x_bounds.1, y_bounds.1 + 1);
    walls.insert((x_bounds.1, y_bounds.1 + 2));

    let mut blizzards: HashMap<i32, HashMap<(i32, i32), HashSet<Direction>>> = HashMap::new();
    for i in 0..=1000 {
        let mut new_blizzards: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
        for (pos, directions) in prev_blizzards.into_iter() {
            for d in directions {
                let new_pos = d.step(pos.clone(), x_bounds, y_bounds);
                let mut new_set = new_blizzards.get(&new_pos).unwrap_or(&HashSet::new()).clone();
                new_set.insert(d.clone());
                new_blizzards.insert(new_pos, new_set);
            }
        }
        prev_blizzards = new_blizzards.clone();
        blizzards.insert(i, new_blizzards);
    }

    let mut cache = HashMap::new();

    let optimal = optimal_moves(&curr_location, goal, &blizzards, &walls, 0, &mut cache,
                                1000,
                                &x_bounds, &y_bounds);

    return optimal.to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day24/part1_test_output.txt");
    let actual = part1("src/day24/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day24/input.txt");
    io::write_to_file("src/day24/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut prev_blizzards: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
    let mut walls: HashSet<(i32, i32)> = HashSet::new();

    let height = s.split("\n").count() as i32;
    let y_bounds = (1, height - 2);
    let width = s.split("\n").collect::<Vec<&str>>()[0].len() as i32;
    let x_bounds = (1, width - 2);

    for (y, line) in s.split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                walls.insert((x as i32, y as i32));
            } else if char != '.' {
                prev_blizzards.insert((x as i32, y as i32), HashSet::from_iter(vec![Direction::new
                    (char)]
                    .into_iter()));
            }
        }
    }

    let start = (x_bounds.0, y_bounds.0 - 1);
    walls.insert((x_bounds.0, y_bounds.0 - 2));
    let goal = (x_bounds.1, y_bounds.1 + 1);
    walls.insert((x_bounds.1, y_bounds.1 + 2));

    let mut blizzards: HashMap<i32, HashMap<(i32, i32), HashSet<Direction>>> = HashMap::new();
    for i in 0..=1000 {
        let mut new_blizzards: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
        for (pos, directions) in prev_blizzards.into_iter() {
            for d in directions {
                let new_pos = d.step(pos.clone(), x_bounds, y_bounds);
                let mut new_set = new_blizzards.get(&new_pos).unwrap_or(&HashSet::new()).clone();
                new_set.insert(d.clone());
                new_blizzards.insert(new_pos, new_set);
            }
        }
        prev_blizzards = new_blizzards.clone();
        blizzards.insert(i, new_blizzards);
    }

    let mut cache = HashMap::new();

    let first_trip_steps = optimal_moves(&start, goal.clone(), &blizzards, &walls, 0, &mut cache,
                                1000,
                                &x_bounds, &y_bounds);

    cache.clear();

    let second_trip_steps = optimal_moves(&goal, start.clone(), &blizzards, &walls, first_trip_steps, &mut cache,
                                         1000,
                                         &x_bounds, &y_bounds);

    cache.clear();

    let final_trip_steps = optimal_moves(&start, goal.clone(), &blizzards, &walls,
                                         second_trip_steps, &mut cache,
                                          1000,
                                          &x_bounds, &y_bounds);

    return final_trip_steps.to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day24/part2_test_output.txt");
    let actual = part2("src/day24/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day24/input.txt");
    io::write_to_file("src/day24/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
