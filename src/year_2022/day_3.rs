use itertools::Itertools;

use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

fn get_common_char(items: &[&str]) -> Option<char> {
    items[0].chars().find(|&letter| {
        for item in items.iter().skip(1) {
            if !item.contains(letter) {
                return false;
            }
        }

        true
    })
}

fn get_char_priority(letter: &char) -> u32 {
    let mut buffer = [0; 1];
    letter.encode_utf8(&mut buffer);

    let byte = buffer[0];

    if letter.is_uppercase() {
        (byte - 38) as u32
    } else {
        (byte - 96) as u32
    }
}

fn get_line_priority(line: &str) -> Result<u32, BoxedError> {
    let (first_half, second_half) = line.trim().split_at(line.len() / 2);
    let halfs = vec![first_half, second_half];

    let common_char = unwrap_or_return_option!(
        get_common_char(&halfs),
        "Failed to find a common character in the line \"{}\"",
        line
    );

    Ok(get_char_priority(&common_char))
}

fn get_group_priority(group: &[&str]) -> Result<u32, BoxedError> {
    let common_char = unwrap_or_return_option!(
        get_common_char(group),
        "Failed to find a common character in the group \"{:#?}\"",
        group
    );

    Ok(get_char_priority(&common_char))
}

pub fn execute(input: &str) -> DayReturnType {
    let mut total_priority = 0;
    let mut group_priority = 0;
    let input = input.trim();

    for line in input.lines() {
        total_priority += unwrap_or_return!(get_line_priority(line))
    }

    for group in input.lines().chunks(3).into_iter() {
        group_priority += unwrap_or_return!(get_group_priority(&group.collect_vec()))
    }

    Ok((total_priority.to_string(), group_priority.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("157", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("70", result);
    }
}
