use std::collections::HashMap;
use std::hash::Hash;
pub use utils::io;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation_tokens: Vec<String>,
    divisible_by: i64,
    monkey_if_divisible: usize,
    monkey_else: usize
}

impl Monkey {
    fn operation(&mut self, old: i64) -> i64 {
        let item1_str = self.operation_tokens[0].as_str();
        let item1 = if item1_str == "old" {old} else {item1_str.parse::<i64>().unwrap()};
        let item2_str = self.operation_tokens[2].as_str();
        let item2 = if item2_str == "old" {old} else {item2_str.parse::<i64>().unwrap()};
        let operation = self.operation_tokens[1].as_str();
        match operation {
            "+" => item1 + item2,
            "*" => item1 * item2,
            "-" => item1 - item2,
            "/" => item1 / item2,
            _ => panic!()
        }
    }

    fn test(&mut self, worry_level: i64) -> usize {
        if worry_level % self.divisible_by == 0 {
            self.monkey_if_divisible
        } else {
            self.monkey_else
        }
    }
}

fn make_monkey(monkey_desc: &str) -> Monkey {
    let mut lines = monkey_desc.split("\n").map(|line| line.trim()).into_iter();

    lines.next();

    let starting_items_desc = lines.next().unwrap();
    let items = starting_items_desc.split_once(": ").unwrap().1.split(",")
        .map(|item| item.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let operation_desc = lines.next().unwrap();
    let operation_tokens = operation_desc.split_once(" = ").unwrap().1.split_whitespace().map
    (|token| token.to_string()).collect::<Vec<String>>();

    let test_condition = lines.next().unwrap();
    let divisible_by = test_condition.trim().rsplit_once(" ").unwrap().1.parse::<i64>().unwrap();
    let if_true = lines.next().unwrap();
    let monkey_if_divisible = if_true.trim().rsplit_once(" ").unwrap().1.parse::<usize>().unwrap();
    let if_false = lines.next().unwrap();
    let monkey_else = if_false.trim().rsplit_once(" ").unwrap().1.parse::<usize>().unwrap();


    Monkey {
        items,
        operation_tokens,
        divisible_by,
        monkey_if_divisible,
        monkey_else
    }
}

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut monkeys: Vec<Monkey> = s.split("\n\n").map(|monkey_desc| make_monkey(monkey_desc))
        .collect();

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..20 {
        for monkey_num in 0..monkeys.len() {
            let monkey = {monkeys.get_mut(monkey_num).unwrap()};
            let mut new_packages: HashMap<usize, Vec<i64>> = HashMap::with_capacity(monkey.items.len
            ());
            while !monkey.items.is_empty() {
                inspections[monkey_num] += 1;
                let mut worry_level = monkey.items.pop().unwrap();
                worry_level = monkey.operation(worry_level) / 3;
                let new_monkey_num = monkey.test(worry_level);
                if !new_packages.contains_key(&new_monkey_num) {
                    new_packages.insert(new_monkey_num, vec![]);
                }
                let mut new_monkey = new_packages.get_mut(&new_monkey_num).unwrap();
                new_monkey.push(worry_level);
            }
            for (&new_monkey, packages) in new_packages.iter() {
                monkeys[new_monkey].items.extend(packages.iter());
            }
        }
    }

    inspections.sort();
    inspections.reverse();

    return (inspections[0] * inspections[1]).to_string();
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day11/part1_test_output.txt");
    let actual = part1("src/day11/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day11/input.txt");
    io::write_to_file("src/day11/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut monkeys: Vec<Monkey> = s.split("\n\n").map(|monkey_desc| make_monkey(monkey_desc))
        .collect();

    let overflow = monkeys.iter().map(|monkey| monkey.divisible_by).fold(1, |acc, curr| {
        acc * curr
    });

    let mut inspections = vec![0i64; monkeys.len()];

    for _round in 0..10000 {
        for monkey_num in 0..monkeys.len() {
            let monkey = {monkeys.get_mut(monkey_num).unwrap()};
            let mut new_packages: HashMap<usize, Vec<i64>> = HashMap::with_capacity(monkey.items.len
            ());
            while !monkey.items.is_empty() {
                inspections[monkey_num] += 1;
                let mut worry_level = monkey.items.pop().unwrap();
                worry_level = monkey.operation(worry_level);
                if worry_level > 0 {
                    worry_level = worry_level % overflow;
                }
                let new_monkey_num = monkey.test(worry_level);
                if !new_packages.contains_key(&new_monkey_num) {
                    new_packages.insert(new_monkey_num, vec![]);
                }
                let mut new_monkey = new_packages.get_mut(&new_monkey_num).unwrap();
                new_monkey.push(worry_level);
            }
            for (&new_monkey, packages) in new_packages.iter() {
                monkeys[new_monkey].items.extend(packages.iter());
            }
        }
    }

    inspections.sort();
    inspections.reverse();

    return (inspections[0] * inspections[1]).to_string();
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day11/part2_test_output.txt");
    let actual = part2("src/day11/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day11/input.txt");
    io::write_to_file("src/day11/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
