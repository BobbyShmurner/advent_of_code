use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

use std::cmp::*;

#[derive(Clone, Eq, PartialEq)]
struct PacketInfo {
    packets: Vec<Packet>,
    input_str: String,
    stripped_str: String,
    depth: usize,
}

impl PartialOrd for PacketInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Packet::compare(&self.packets, &other.packets)
    }
}

impl std::fmt::Debug for PacketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.input_str)
    }
}

impl PacketInfo {
    fn parse(line: &str) -> Result<Self, BoxedError> {
        let input_str = line.trim();

        let packet = match Packet::parse(input_str)? {
            Packet::Packet(val) => val,
            _ => return_err!("Invalid Packet \"{}\"", input_str),
        };

        let depth = input_str.matches('[').count();
        let stripped_str = input_str
            .replace(['[', ']', ','], "")
            .split_whitespace()
            .collect();

        Ok(Self {
            input_str: input_str.to_string(),
            packets: packet,
            stripped_str,
            depth,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Num(usize),
    Packet(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Packet(left), Packet::Packet(right)) => Packet::compare(left, right),
            _ => None,
        }
    }
}

impl Packet {
    fn parse(line: &str) -> Result<Self, BoxedError> {
        let mut line = line.trim()[1..line.len() - 1].to_string();
        let mut inner_packets = Vec::new();
        let mut packet = Vec::new();

        while line.contains('[') {
            let mut depth = 0;
            let mut range = 0..line.len();

            for (i, character) in line.chars().enumerate() {
                if character == '[' {
                    depth += 1;

                    if depth == 1 {
                        range.start = i;
                    }
                } else if character == ']' {
                    depth -= 1;

                    if depth == 0 {
                        range.end = i + 1;
                        break;
                    }
                }
            }

            inner_packets.insert(0, Packet::parse(&line[range.clone()])?);
            line.replace_range(range.start..range.end, "@");
        }

        for item in line.split(',') {
            if item.is_empty() {
                break;
            }

            if item == "@" {
                packet.push(inner_packets.pop().unwrap());
                continue;
            }

            let item: usize = unwrap_or_return!(
                item.trim().parse(),
                "{} is not a valid item for a packet!",
                item
            );
            packet.push(Packet::Num(item));
        }

        Ok(Packet::Packet(packet))
    }

    fn compare(left: &Vec<Packet>, right: &Vec<Packet>) -> Option<Ordering> {
        let mut i: i32 = -1;

        loop {
            i += 1;

            if i as usize >= left.len() {
                return if left.len() == right.len() {
                    None
                } else {
                    Some(Ordering::Less)
                };
            }

            if i as usize >= right.len() {
                return Some(Ordering::Greater);
            }

            let left_item = &left[i as usize];
            let right_item = &right[i as usize];

            let new_list;

            let (left_item, right_item) = match (left_item, right_item) {
                (Packet::Num(num), Packet::Packet(_)) => {
                    new_list = Packet::Packet(vec![Packet::Num(*num)]);
                    (&new_list, right_item)
                }
                (Packet::Packet(_), Packet::Num(num)) => {
                    new_list = Packet::Packet(vec![Packet::Num(*num)]);
                    (left_item, &new_list)
                }
                _ => (left_item, right_item),
            };

            match (left_item, right_item) {
                (Packet::Num(left), Packet::Num(right)) => {
                    if left == right {
                        continue;
                    }

                    return if left < right {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    };
                }
                (Packet::Packet(left), Packet::Packet(right)) => {
                    let result = Packet::compare(left, right);

                    if result.is_none() {
                        continue;
                    }

                    return result;
                }
                _ => return None,
            }
        }
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let mut correct_order: u32 = 0;
    let mut packets = Vec::new();

    for (i, pair) in input
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
    {
        let pair: Vec<&str> = pair.trim().lines().collect();

        let left = PacketInfo::parse(pair[0])?;
        let right = PacketInfo::parse(pair[1])?;

        packets.push(left.clone());
        packets.push(right.clone());

        if let Some(Ordering::Less) = left.partial_cmp(&right) {
            correct_order += i as u32 + 1;
        }
    }

    packets.push(PacketInfo::parse("[[2]]")?);
    packets.push(PacketInfo::parse("[[6]]")?);

    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut decoder_key = packets
        .iter()
        .position(|packet| packet.input_str == "[[2]]")
        .unwrap()
        + 1;

    decoder_key *= packets
        .iter()
        .position(|packet| packet.input_str == "[[6]]")
        .unwrap()
        + 1;

    Ok((correct_order.to_string(), decoder_key.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("13", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("140", result);
    }
}
