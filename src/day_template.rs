use simple_error::SimpleError;

use crate::macros::*;
use crate::DayReturnType;

pub fn execute(input: &str) -> DayReturnType {
    return_err!("Code For This Day Is Not Complete!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#""#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("Answer1", result);
    }

    #[test]
    fn part2_example() {
        let input = r#""#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("Answer2", result);
    }
}
