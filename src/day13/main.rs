use std::cmp::Ordering;
pub use utils::io;

#[derive(Debug)]
enum PacketData {
    List(Vec<PacketData>),
    Num(i32),
}

impl PartialEq<Self> for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match self {
            List(packets) => {
                if let List(other_packets) = other {
                    packets.eq(other_packets)
                } else {
                    false
                }
            },
            Num(num) => {
                if let Num(other_num) = other {
                    num.eq(other_num)
                } else {
                    false
                }
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }
        return match self {
            List(packets) => {
                match other {
                    List(other_packets) => {
                        for (p1, p2) in packets.iter().zip(other_packets.iter()) {
                            match p1.partial_cmp(p2) {
                                Some(Ordering::Less) => { return Some(Ordering::Less) },
                                Some(Ordering::Equal) => {},
                                Some(Ordering::Greater) => { return Some(Ordering::Greater) },
                                _ => panic!()
                            }
                        }
                        packets.len().partial_cmp(&other_packets.len())
                    },
                    Num(other_num) => {
                        self.partial_cmp(&List(vec![Num(*other_num)]))
                    }
                }
            },
            Num(num) => {
                match other {
                    List(_other_packets) => {
                        List(vec![Num(*num)]).partial_cmp(&other)
                    },
                    Num(other_num) => {
                        num.partial_cmp(other_num)
                    }
                }
            }
        }
    }
}

use PacketData::{List, Num};

fn parse_packet(line: &str) -> PacketData {
    let mut vec: Vec<PacketData> = vec![];
    let mut line_iter = line.trim().chars().into_iter().enumerate();
    while let Some((i, c)) = line_iter.next() {
        if i == 0 || i == line.trim().len() - 1 {
            continue;
        }
        if c.is_digit(10) {
            let mut num = c.to_string();
            while let Some((_i, next_char)) = line_iter.next() {
                if next_char.is_digit(10) {
                    num.push(next_char);
                } else {
                    if !num.is_empty() {
                        vec.push(Num(num.parse().unwrap()));
                        num.clear();
                    }
                    if next_char == '[' {
                        let mut level = 1;
                        let mut subpacket = c.to_string();
                        while level > 0 {
                            let (j, next_next_char) = line_iter.next().unwrap_or((0, ' '));
                            if next_next_char == '[' {
                                level += 1;
                            } else if next_next_char == ']' {
                                level -= 1;
                            }
                            subpacket.push(next_next_char);
                        }
                        vec.push(parse_packet(subpacket.as_str()));
                    } else if next_char == ']' {
                        return List(vec);
                    }
                }
            }
        } else if c == '[' {
            let mut level = 1;
            let mut subpacket = c.to_string();
            while level > 0 {
                let (j, next_char) = line_iter.next().unwrap_or((0, ' '));
                if next_char == '[' {
                    level += 1;
                } else if next_char == ']' {
                    level -= 1;
                }
                subpacket.push(next_char);
            }
            vec.push(parse_packet(subpacket.as_str()));
        } else if c == ']' {
            return List(vec);
        }
    }
    List(vec)
}

fn part1(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    s.split("\n\n").enumerate().map(|(i, two_packets)| -> usize {
        let (packet_1_str, packet_2_str) = two_packets.split_once("\n").unwrap();
        let (packet_1, packet_2) = (parse_packet(packet_1_str), parse_packet(packet_2_str));

        match packet_1.partial_cmp(&packet_2).unwrap() {
            Ordering::Less => {i + 1}
            Ordering::Equal => {0}
            Ordering::Greater => {0}
        }
    }).sum::<usize>().to_string()
}

#[test]
fn test_part1() {
    let expected = io::file_to_str("src/day13/part1_test_output.txt");
    let actual = part1("src/day13/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 1 passed!");
}

fn run_part1() {
    let actual = part1("src/day13/input.txt");
    io::write_to_file("src/day13/part1_output.txt", actual);
}

fn part2(filename: &str) -> String {
    let s: String = io::file_to_str(filename);

    let divider1 = parse_packet("[[2]]");
    let divider2 = parse_packet("[[6]]");

    let mut packets = s.split("\n").filter(|line| !line.trim().is_empty()).map(|line| parse_packet
        (line)).collect::<Vec<PacketData>>();
    packets.push(parse_packet("[[2]]"));
    packets.push(parse_packet("[[6]]"));

    packets.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());

    let div1_pos = 1 + packets.iter().position(|packet| packet == &divider1).unwrap();
    let div2_pos = 1 + packets.iter().position(|packet| packet == &divider2).unwrap();
    (div1_pos * div2_pos).to_string()
}

#[test]
fn test_part2() {
    let expected = io::file_to_str("src/day13/part2_test_output.txt");
    let actual = part2("src/day13/test_input.txt");

    assert_eq!(expected, actual, "Expected {} but received {}", expected, actual);
    println!("Test for part 2 passed!");
}

fn run_part2() {
    let actual = part2("src/day13/input.txt");
    io::write_to_file("src/day13/part2_output.txt", actual);
}

fn main() {
    run_part2();
}
