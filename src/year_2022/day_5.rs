use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

use lazy_static::lazy_static;
use regex::Regex;

struct Crates {
    columns: Vec<Vec<char>>,
}

struct MoveInstruction {
    target: usize,
    destination: usize,
    amount: usize,
}

impl MoveInstruction {
    fn new(input: &str) -> Result<Self, BoxedError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"move (?P<amount>\d+) from (?P<target>\d+) to (?P<destination>\d+)")
                    .unwrap();
        }

        let captures = unwrap_option_or_return!(
            RE.captures(input.trim()),
            "Invalid Move Instruction: \"{}\"",
            input.trim()
        );

        Ok(Self {
            target: captures.name("target").unwrap().as_str().parse().unwrap(),
            destination: captures
                .name("destination")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            amount: captures.name("amount").unwrap().as_str().parse().unwrap(),
        })
    }

    fn parse_many(lines: &str) -> Result<Vec<Self>, BoxedError> {
        let mut instructions = Vec::new();

        for instruction in lines.trim().lines() {
            instructions.push(MoveInstruction::new(instruction)?);
        }

        Ok(instructions)
    }
}

impl Crates {
    fn new(lines: &str) -> Result<Self, BoxedError> {
        let mut lines = lines.lines().rev();
        lines.next();

        let bottom_line =
            unwrap_option_or_return!(lines.next(), "Invalid Input: No inital crate setup found!");
        let line_length = bottom_line.len();

        let lines: Vec<&str> = lines.collect();
        let mut columns = Vec::new();

        for x in (1..line_length).step_by(4) {
            let mut column = Vec::new();

            for line in &lines {
                if x > line.len() {
                    continue;
                }

                let crate_val = line.chars().nth(x).expect("Index Out Of Bound!");
                if crate_val == ' ' {
                    break;
                }

                column.push(crate_val);
            }

            columns.push(column);
        }

        Ok(Self { columns })
    }

    fn move_crates(
        &mut self,
        instruction: &MoveInstruction,
        at_once: bool,
    ) -> Result<(), BoxedError> {
        let target_column = unwrap_option_or_return!(
            self.columns.get_mut(instruction.target - 1),
            "Invalid target column \"{}\"",
            instruction.target
        );
        let mut boxes_to_move = Vec::new();

        for _ in 0..instruction.amount {
            boxes_to_move.push(unwrap_option_or_return!(
                target_column.pop(),
                "Cannot move {} box(es) from column {} because there aren't any boxes left!",
                instruction.amount,
                instruction.target
            ));
        }

        let dest_column = unwrap_option_or_return!(
            self.columns.get_mut(instruction.destination - 1),
            "Invalid destination column \"{}\"",
            instruction.destination
        );

        if at_once {
            boxes_to_move.reverse()
        }

        dest_column.append(&mut boxes_to_move);
        Ok(())
    }

    fn get_answer(self) -> String {
        let mut answer = String::new();

        for mut column in self.columns {
            answer.push(column.pop().unwrap());
        }

        answer
    }
}

impl std::fmt::Display for Crates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = Vec::new();
        let mut column_indecies = String::new();

        if self.columns.len() > 9 {
            panic!("Invalid Column Size!")
        }

        for i in 1..=self.columns.len() {
            column_indecies.push_str(&format!(" {i}  "));
        }
        column_indecies.pop();
        lines.push(column_indecies);

        for i in 0..usize::MAX {
            let mut line = String::new();
            let mut line_has_column = false;

            for column in &self.columns {
                if i >= column.len() {
                    line.push_str("    ");
                    continue;
                }

                line_has_column = true;
                line.push_str(&format!("[{}] ", column[i]));
            }

            if !line_has_column {
                break;
            }

            line.pop();
            lines.push(line);
        }

        lines.reverse();
        for line in lines {
            f.write_str(&format!("{line}\n"))?;
        }

        Ok(())
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let (inital_setup, move_instructions) = unwrap_option_or_return!(
        input.split_once("move"),
        "Invalid Input: No split between the inital crate setup and the instructions!"
    );
    let move_instructions = "move".to_string() + move_instructions;

    let mut crates_part1 = Crates::new(inital_setup)?;
    let mut crates_part2 = Crates::new(inital_setup)?;

    let instructions = MoveInstruction::parse_many(&move_instructions)?;

    for instruction in instructions {
        crates_part1.move_crates(&instruction, false)?;
        crates_part2.move_crates(&instruction, true)?;
    }

    Ok((crates_part1.get_answer(), crates_part2.get_answer()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("CMZ", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("MCD", result);
    }
}
