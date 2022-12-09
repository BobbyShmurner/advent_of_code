use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

#[derive(Debug)]
struct Elf {
    calories: u32,
}

impl Elf {
    fn new(lines: &str) -> Result<Elf, BoxedError> {
        let mut total_calories = 0;

        for line in lines.split('\n') {
            let line = line.trim();
            let calories: u32 = unwrap_or_return!(
                line.parse(),
                "\"{}\" isn't a valid value for calories!",
                line
            );

            total_calories += calories;
        }

        Ok(Elf {
            calories: total_calories,
        })
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let mut top_three_elfs: Vec<Elf> = Vec::new();

    for elf_data in input.split("\n\n") {
        let new_elf = unwrap_or_return!(Elf::new(elf_data));

        if top_three_elfs.is_empty() {
            top_three_elfs.push(new_elf);
            continue;
        }

        for (i, elf) in top_three_elfs.iter().enumerate() {
            if top_three_elfs.len() < 3 || new_elf.calories > elf.calories {
                top_three_elfs.insert(i, new_elf);
                break;
            }
        }

        if top_three_elfs.len() > 3 {
            top_three_elfs.pop();
        }
    }

    let most_calories = top_three_elfs[0].calories;
    let mut top_three_calories = 0;

    for elf in top_three_elfs {
        top_three_calories += elf.calories;
    }

    Ok((most_calories.to_string(), top_three_calories.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("24000", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("45000", result);
    }
}
