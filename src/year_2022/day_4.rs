use std::ops::RangeInclusive;

use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

fn str_to_range(input: &str) -> Result<RangeInclusive<u32>, BoxedError> {
    let split = input.split('-').collect::<Vec<&str>>();

    if split.len() != 2 {
        return_err!("Invalid range \"{}\"", input.trim());
    }

    let first = split[0];
    let second = split[1];

    let start: u32 = unwrap_or_return!(
        first.trim().parse(),
        "\"{}\" is not a valid number for a range!",
        first
    );

    let end: u32 = unwrap_or_return!(
        second.trim().parse(),
        "\"{}\" is not a valid number for a range!",
        second
    );

    Ok(RangeInclusive::new(start, end))
}

fn line_to_ranges(line: &str) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>), BoxedError> {
    let split = line.trim().split(',').collect::<Vec<&str>>();

    if split.len() != 2 {
        return_err!("Invalid line \"{}\"", line.trim());
    }

    let first_str = split[0];
    let second_str = split[1];

    let first = str_to_range(first_str)?;
    let second = str_to_range(second_str)?;

    Ok((first, second))
}

fn line_to_ordered_vecs(line: &str) -> Result<(Vec<u32>, Vec<u32>), BoxedError> {
    let (first_range, second_range) = line_to_ranges(line)?;
    let first = first_range.collect::<Vec<u32>>();
    let second = second_range.collect::<Vec<u32>>();

    if first.len() > second.len() {
        Ok((first, second))
    } else {
        Ok((second, first))
    }
}

fn all_simular_in_line(line: &str) -> Result<bool, BoxedError> {
    let (larger, smaller) = line_to_ordered_vecs(line)?;

    Ok(smaller.iter().all(|num| larger.contains(num)))
}

fn any_simular_in_line(line: &str) -> Result<bool, BoxedError> {
    let (larger, smaller) = line_to_ordered_vecs(line)?;

    Ok(smaller.iter().any(|num| larger.contains(num)))
}

pub fn execute(input: &str) -> DayReturnType {
    let mut all_overlaps = 0;
    let mut any_overlaps = 0;

    for line in input.trim().lines() {
        if unwrap_or_return!(all_simular_in_line(line)) {
            all_overlaps += 1;
        }

        if unwrap_or_return!(any_simular_in_line(line)) {
            any_overlaps += 1;
        }
    }

    Ok((all_overlaps.to_string(), any_overlaps.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("2", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("4", result);
    }
}
