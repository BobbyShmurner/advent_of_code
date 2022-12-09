use crate::macros::*;
use crate::DayReturnType;

pub fn execute(_input: &str) -> DayReturnType {
    return_err!("Code For This Day Is Not Complete!");
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn part1_example() {
//         let input = r#"    [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2"#;

//         let result = super::execute(input).unwrap().0;
//         assert_eq!("CMZ", result);
//     }

//     #[test]
//     fn part2_example() {
//         let input = r#"    [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2"#;

//         let result = super::execute(input).unwrap().1;
//         assert_eq!("MCD", result);
//     }
// }
