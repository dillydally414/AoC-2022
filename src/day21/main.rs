use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
pub use utils::io;

#[derive(Debug)]
enum MonkeyVal {
    Composed(String, String, String),
    Value(f64)
}

fn find_monkey_value(monkey: String, monkeys: &HashMap<String, MonkeyVal>, monkey_values:
&mut HashMap<String, f64>) -> f64 {
    if monkey_values.contains_key(&monkey) {
        return monkey_values.get(&monkey).unwrap().clone();
    }
    let val = match monkeys.get(&monkey).unwrap() {
        MonkeyVal::Composed(monkey1, op, monkey2) => {
            let monkey1_val = find_monkey_value(monkey1.clone(), monkeys, monkey_values);
            let monkey2_val = find_monkey_value(monkey2.clone(), monkeys, monkey_values);
            match op.as_str() {
                "*" => monkey1_val * monkey2_val,
                "/" => monkey1_val / monkey2_val,
                "+" => monkey1_val + monkey2_val,
                "-" => monkey1_val - monkey2_val,
                _ => unreachable!()
            }
        },
        MonkeyVal::Value(num) => num.clone()
    };
    monkey_values.insert(monkey, val);
    val
}

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut monkeys = HashMap::from_iter(s.split("\n").into_iter().map(|line: &str| -> (String,
                                                                                        MonkeyVal) {
        let (monkey_name, monkey_val) = line.split_once(": ").unwrap();
        let monkey_val = if monkey_val.contains(" ") {
            let mut tokens = monkey_val.split_whitespace();
            MonkeyVal::Composed(tokens.next().unwrap().to_string(), tokens.next().unwrap()
                .to_string(),tokens.next().unwrap().to_string())
        } else {
            MonkeyVal::Value(monkey_val.parse().unwrap())
        };
        (monkey_name.to_string(), monkey_val)
    }));

    let mut monkey_values: HashMap<String, f64> = HashMap::new();

    find_monkey_value(String::from("root"), &monkeys, &mut monkey_values).to_string()
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day21/part1_test_output.txt");
    let actual = part1("src/day21/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day21/input.txt");
    io::write_to_file("src/day21/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let mut root_children: (String, String) = (String::new(), String::new());

    let mut monkeys = HashMap::from_iter(s.split("\n").into_iter().map(|line: &str| -> (String,
                                                                                        MonkeyVal) {
        let (monkey_name, monkey_val) = line.split_once(": ").unwrap();
        let monkey_val = if monkey_val.contains(" ") {
            let tokens: Vec<&str> = monkey_val.split_whitespace().collect();
            let composed = MonkeyVal::Composed(tokens[0].to_string(), tokens[1].to_string(),
                                               tokens[2].to_string());
            if monkey_name == "root" {
                root_children = (tokens[0].to_string(), tokens[2].to_string());
            }
            composed
        } else {
            MonkeyVal::Value(monkey_val.parse().unwrap())
        };
        (monkey_name.to_string(), monkey_val)
    }));

    let mut monkey_values: HashMap<String, f64> = HashMap::new();

    let mut high = 100000000000000.0;
    let mut low = 0.0;

    loop {
        monkeys.insert(String::from("humn"), MonkeyVal::Value(high));
        monkey_values.clear();
        let val1_high = find_monkey_value(String::from(root_children.0.to_string()), &monkeys, &mut
            monkey_values);
        let val2_high = find_monkey_value(String::from(root_children.1.to_string()), &monkeys, &mut
            monkey_values);
        let diff_high = val1_high - val2_high;

        let middle = (high + low) / 2.0;
        monkeys.insert(String::from("humn"), MonkeyVal::Value(middle));
        monkey_values.clear();
        let val1_mid = find_monkey_value(String::from(root_children.0.to_string()), &monkeys, &mut
            monkey_values);
        let val2_mid = find_monkey_value(String::from(root_children.1.to_string()), &monkeys, &mut
            monkey_values);
        let diff_mid = val1_mid - val2_mid;

        if diff_mid == 0.0 {
            return middle.to_string();
        }

        if diff_high.signum() == diff_mid.signum() {
            high = middle;
        } else {
            low = middle;
        }

        println!("{} {} {}", low, middle, high);
    }
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day21/part2_test_output.txt");
    let actual = part2("src/day21/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day21/input.txt");
    io::write_to_file("src/day21/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
