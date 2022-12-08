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
