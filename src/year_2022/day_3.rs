use simple_error::SimpleError;

use crate::DayReturnType;

pub fn execute(input: &str) -> DayReturnType {
    Err(Box::new(SimpleError::new(
        "Code For This Day Is Not Complete!",
    )))
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
