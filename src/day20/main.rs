use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;
pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut nums: Vec<i32> = s.split("\n").map(|line| line.trim().parse::<i32>().unwrap()).collect();

    let queue = nums.clone();

    let len = nums.len();

    let mut touched = vec![false; len];

    for num in queue {
        let pos = nums.iter().enumerate().position(|(idx, j)| *j == num && !touched[idx]).unwrap();
        let new_pos = ((pos as i32 + num + (len - 1) as i32 * 1000)
            % ((len - 1) as i32)) as usize;
        touched.remove(pos);
        nums.remove(pos);
        if new_pos == len - 1 || new_pos == 0 {
            touched.push(true);
            nums.push(num);
        } else {
            touched.insert(new_pos, true);
            nums.insert(new_pos,  num);
        }
    }

    let zero_pos = nums.iter().position(|j| *j == 0).unwrap();

    vec![1000, 2000, 3000].into_iter().map(|pos| (pos + zero_pos) % len).map
        (|pos| nums[pos]).sum::<i32>().to_string()
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day20/part1_test_output.txt");
    let actual = part1("src/day20/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day20/input.txt");
    io::write_to_file("src/day20/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    const DECRYPTION_KEY: i64 = 811589153;

    let mut nums: Vec<i64> = s.split("\n").map(|line| line.trim().parse::<i64>().unwrap() *
        DECRYPTION_KEY).collect_vec();

    let queue = nums.clone();

    let len = nums.len();

    let mut order = (0..len).collect_vec();

    for (i, num) in queue.repeat(10).into_iter().enumerate() {
        let pos = order.iter().position(|order_num| order_num == i % len).unwrap();
        let new_pos = ((pos as i64 + num + (len - 1) as i64 * 10000000000)
            % ((len - 1) as i64)) as usize;
        order.remove(pos);
        nums.remove(pos);
        if new_pos == len - 1 || new_pos == 0 {
            order.push(i % len);
            nums.push(num);
        } else {
            order.insert(new_pos, i % len);
            nums.insert(new_pos,  num);
        }
    }

    let zero_pos = nums.iter().position(|j| *j == 0).unwrap();

    vec![1000, 2000, 3000].into_iter().map(|pos| (pos + zero_pos) % len).map
    (|pos| nums[pos]).sum::<i64>().to_string()
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day20/part2_test_output.txt");
    let actual = part2("src/day20/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day20/input.txt");
    io::write_to_file("src/day20/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
