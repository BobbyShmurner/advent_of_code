use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

#[derive(PartialEq)]
enum IsCorrectOrder {
    Correct,
    Incorrect,
    Unsure,
}

#[derive(Debug)]
enum Packet {
    Num(usize),
    Packet(Vec<Packet>),
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

    fn are_packets_ordered_correctly(
        left: &mut Vec<Self>,
        right: &mut Vec<Self>,
    ) -> IsCorrectOrder {
        let mut i: i32 = -1;

        loop {
            i += 1;

            if i as usize >= left.len() {
                return if left.len() == right.len() {
                    IsCorrectOrder::Unsure
                } else {
                    IsCorrectOrder::Correct
                };
            }

            if i as usize >= right.len() {
                return IsCorrectOrder::Incorrect;
            }

            let left_item = &mut left[i as usize];
            let right_item = &mut right[i as usize];

            match (left_item, right_item) {
                (Packet::Num(left), Packet::Num(right)) => {
                    if left == right {
                        continue;
                    }

                    return if left < right {
                        IsCorrectOrder::Correct
                    } else {
                        IsCorrectOrder::Incorrect
                    };
                }
                (Packet::Num(num), Packet::Packet(_)) => {
                    left[i as usize] = Packet::Packet(vec![Packet::Num(*num)]);
                    i -= 1;
                    continue;
                }
                (Packet::Packet(_), Packet::Num(num)) => {
                    right[i as usize] = Packet::Packet(vec![Packet::Num(*num)]);
                    i -= 1;
                    continue;
                }
                (Packet::Packet(left), Packet::Packet(right)) => {
                    let result = Packet::are_packets_ordered_correctly(left, right);

                    if result == IsCorrectOrder::Unsure {
                        continue;
                    }

                    return result;
                }
            }
        }
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let mut correct_order: u32 = 0;
    for (i, pair) in input
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
    {
        let pair: Vec<&str> = pair.trim().lines().collect();

        let mut left = if let Packet::Packet(left) = Packet::parse(pair[0])? {
            left
        } else {
            return_err!("Invalid packet \"{:?}\"", pair[0]);
        };

        let mut right = if let Packet::Packet(right) = Packet::parse(pair[1])? {
            right
        } else {
            return_err!("Invalid packet \"{:?}\"", pair[1]);
        };

        if Packet::are_packets_ordered_correctly(&mut left, &mut right) == IsCorrectOrder::Correct {
            correct_order += i as u32 + 1;
        }
    }

    Ok((correct_order.to_string(), "Not Implemented".to_string()))
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
        assert_eq!("Not Implemented", result);
    }
}
