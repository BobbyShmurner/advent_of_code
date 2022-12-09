use simple_error::SimpleError;

use crate::DayReturnType;

pub fn execute(_input: &str) -> DayReturnType {
    Err(Box::new(SimpleError::new(
        "Code For This Day Is Not Complete!",
    )))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn part1_example() {
//         let test_data = vec![
//             ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7"),
//             ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5"),
//             ("nppdvjthqldpwncqszvftbrmjlhg", "6"),
//             ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"),
//             ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"),
//         ];

//         for (input, answer) in test_data {
//             let result = super::execute(input).unwrap().0;
//             assert_eq!(answer, result);
//         }
//     }

//     #[test]
//     fn part2_example() {
//         let test_data = vec![
//             ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19"),
//             ("bvwbjplbgvbhsrlpgdmjqwftvncz", "23"),
//             ("nppdvjthqldpwncqszvftbrmjlhg", "23"),
//             ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"),
//             ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"),
//         ];

//         for (input, answer) in test_data {
//             let result = super::execute(input).unwrap().1;
//             assert_eq!(answer, result);
//         }
//     }
// }
