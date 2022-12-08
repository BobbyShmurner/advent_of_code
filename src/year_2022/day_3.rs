use simple_error::SimpleError;

use crate::DayReturnType;

fn get_common_char(input: &str) -> Option<char> {
    let (first_half, second_half) = input.trim().split_at(input.len() / 2);

    first_half
        .chars()
        .find(|&letter| second_half.contains(letter))
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

fn get_line_priority(line: &str) -> Result<u32, String> {
    let common_char = match get_common_char(line) {
        Some(val) => val,
        None => {
            return Err(format!(
                "Failed to find a common character in the line \"{}\"",
                line
            ))
        }
    };

    Ok(get_char_priority(&common_char))
}

pub fn execute(input: &str) -> DayReturnType {
    let mut total_priority = 0;

    for line in input.trim().lines() {
        total_priority += match get_line_priority(line) {
            Ok(val) => val,
            Err(e) => return Err(Box::new(SimpleError::new(e))),
        }
    }

    Ok((total_priority.to_string(), "Answer2".to_string()))
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
