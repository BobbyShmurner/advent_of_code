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
//         let input = r#"$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k"#;

//         let result = super::execute(input).unwrap().0;
//         assert_eq!("95437", result);
//     }

//     #[test]
//     fn part2_example() {
//         let input = r#"$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k"#;

//         let result = super::execute(input).unwrap().1;
//         assert_eq!("24933642", result);
//     }
// }
