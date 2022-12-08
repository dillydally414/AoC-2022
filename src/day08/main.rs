pub use utils::io;

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let trees: Vec<Vec<u32>> = s.split('\n').map(|line| line.chars().map(|c| c.to_digit(10)
        .unwrap()).collect()).collect();

    trees.iter().enumerate().map(|(row_idx, row)| -> u32 {
        row.iter().enumerate().map(|(col_idx, &tree)| -> u32 {
            let top = row_idx == 0 || (
                (0..row_idx).into_iter().all(|r_idx| trees[r_idx][col_idx] < tree)
            );
            let bottom = row_idx == trees.len() - 1 || (
                (row_idx + 1..trees.len()).into_iter().all(|r_idx| trees[r_idx][col_idx] < tree)
            );
            let left = col_idx == 0 || (
                (0..col_idx).into_iter().all(|c_idx| trees[row_idx][c_idx] < tree)
            );
            let right = col_idx == row.len() - 1 || (
                (col_idx + 1..trees.len()).into_iter().all(|c_idx| trees[row_idx][c_idx] < tree)
            );
            if top || bottom || left || right {1} else {0}
        }).sum()
    }).sum::<u32>().to_string()
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day08/part1_test_output.txt");
    let actual = part1("src/day08/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day08/input.txt");
    io::write_to_file("src/day08/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let trees: Vec<Vec<u32>> = s.split('\n').map(|line| line.chars().map(|c| c.to_digit(10)
        .unwrap()).collect()).collect();

    trees.iter().enumerate().map(|(row_idx, row)| -> usize {
        if row_idx == 0 || row_idx == trees.len() - 1 {
            return 0
        }
        row.iter().enumerate().map(|(col_idx, &tree)| -> usize {
            if col_idx == 0 || col_idx == row.len() - 1 {
                return 0
            }
            let top_score = (1..row_idx).rev().into_iter().fold(1, |acc, r_idx| {
                return if acc != row_idx - r_idx || trees[r_idx][col_idx] >= tree {
                    acc
                } else {
                    acc + 1
                }
            });
            let bot_score = (row_idx + 1..trees.len() - 1).into_iter().fold(1, |acc, r_idx| {
                return if acc != r_idx - row_idx || trees[r_idx][col_idx] >= tree {
                    acc
                } else {
                    acc + 1
                }
            });
            let left_score = (1..col_idx).rev().into_iter().fold(1, |acc, c_idx| {
                return if acc != col_idx - c_idx || trees[row_idx][c_idx] >= tree {
                    acc
                } else {
                    acc + 1
                }
            });
            let right_score = (col_idx + 1..row.len() - 1).into_iter().fold(1, |acc, c_idx| {
                return if acc != c_idx - col_idx || trees[row_idx][c_idx] >= tree {
                    acc
                } else {
                    acc + 1
                }
            });
            top_score * left_score * bot_score * right_score
        }).max().unwrap()
    }).max().unwrap().to_string()
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day08/part2_test_output.txt");
    let actual = part2("src/day08/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day08/input.txt");
    io::write_to_file("src/day08/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
