use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

#[derive(PartialEq)]
enum PlayableItems {
    Rock,
    Paper,
    Scissors,
}

enum RoundStates {
    Win,
    Loose,
    Draw,
}

impl RoundStates {
    fn from_letter(letter: &str) -> Result<Self, BoxedError> {
        match letter.to_uppercase().trim() {
            "X" => Ok(Self::Loose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => return_err!("Invalid letter \"{}\"", letter),
        }
    }
}

impl PlayableItems {
    fn from_letter(letter: &str) -> Result<Self, BoxedError> {
        match letter.to_uppercase().trim() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => return_err!("Invalid letter \"{}\"", letter),
        }
    }

    fn from_round_state<'a>(other: &'a Self, state: &RoundStates) -> &'a Self {
        match state {
            RoundStates::Win => other.get_loosing_item(),
            RoundStates::Loose => other.get_winning_item(),
            RoundStates::Draw => other,
        }
    }

    fn get_winning_item(&self) -> &Self {
        match self {
            Self::Rock => &Self::Scissors,
            Self::Paper => &Self::Rock,
            Self::Scissors => &Self::Paper,
        }
    }

    fn get_loosing_item(&self) -> &Self {
        match self {
            Self::Rock => &Self::Paper,
            Self::Paper => &Self::Scissors,
            Self::Scissors => &Self::Rock,
        }
    }

    fn get_item_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_round_score(&self, other: &Self) -> u32 {
        let item_score = self.get_item_score();

        let round_score = if other == self {
            3
        } else if other == self.get_winning_item() {
            6
        } else {
            0
        };

        item_score + round_score
    }

    fn get_round_from_str(input: &str) -> Result<(u32, u32), BoxedError> {
        let (other_str, this_str) = input.trim().split_at(1);

        let other = PlayableItems::from_letter(other_str)?;
        let round_state = RoundStates::from_letter(this_str)?;

        let part_1 = PlayableItems::from_letter(this_str)?;
        let part_2 = PlayableItems::from_round_state(&other, &round_state);

        Ok((
            part_1.get_round_score(&other),
            part_2.get_round_score(&other),
        ))
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in input.trim().lines() {
        let (part1_score, part2_score) = unwrap_or_return!(PlayableItems::get_round_from_str(line));

        part1_total += part1_score;
        part2_total += part2_score;
    }

    Ok((part1_total.to_string(), part2_total.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"A Y
B X
C Z"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("15", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"A Y
B X
C Z"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("12", result);
    }
}
