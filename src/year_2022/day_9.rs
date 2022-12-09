use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn direction(dir: &Direction) -> Self {
        match *dir {
            Direction::Up => Self::new(0, 1),
            Direction::Down => Self::new(0, -1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }

    fn squared_distance(&self, other: &Self) -> i32 {
        if self == other {
            return 0;
        }

        (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Rope {
    parts: Vec<Pos>,
    unique_tail_positions: Vec<Pos>,
}

impl Rope {
    fn new(length: usize) -> Result<Self, BoxedError> {
        if length < 2 {
            return_err!(
                "Cannot create a rope of size {}, ropes must have a length of at least 2!",
                length
            );
        }

        Ok(Self {
            parts: vec![Pos::zero(); length],
            unique_tail_positions: vec![Pos::zero()],
        })
    }

    fn move_part(&mut self, i: usize, step: Pos) {
        {
            let part = self.parts.get_mut(i).unwrap();
            *part += step;
        }

        let part = self.parts[i];

        if i == self.parts.len() - 1 {
            if !self.unique_tail_positions.contains(&part) {
                self.unique_tail_positions.push(part);
            }

            return;
        }

        let next_part = self.parts[i + 1];
        let square_distance = part.squared_distance(&next_part);
        let delta = part - next_part;

        if square_distance < 4 {
            return;
        }

        self.move_part(i + 1, delta.signum());
    }

    fn move_head(&mut self, direction: Direction, step_size: i32) {
        let step = Pos::direction(&direction);

        for _step_index in 0..step_size {
            self.move_part(0, step);
        }
    }

    fn move_using_str(&mut self, line: &str) -> Result<(), BoxedError> {
        let (lhs, rhs) = line.trim().split_at(1);

        let direction = match lhs.to_uppercase().trim() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return_err!("Invalid Direction \"{}\"", lhs.trim()),
        };

        let step_size: i32 =
            unwrap_or_return!(rhs.trim().parse(), "Invalid Step Size \"{}\"", rhs.trim());

        self.move_head(direction, step_size);

        Ok(())
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let mut rope_part_1 = unwrap_or_return!(Rope::new(2));
    let mut rope_part_2 = unwrap_or_return!(Rope::new(10));

    for line in input.trim().lines() {
        rope_part_1.move_using_str(line.trim())?;
        rope_part_2.move_using_str(line.trim())?;
    }

    Ok((
        rope_part_1.unique_tail_positions.len().to_string(),
        rope_part_2.unique_tail_positions.len().to_string(),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("13", result);
    }

    #[test]
    fn part2_example() {
        let test_data = vec![
            (
                r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
                "1",
            ),
            (
                r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
                "36",
            ),
        ];

        for (input, answer) in test_data {
            let result = super::execute(input).unwrap().1;
            assert_eq!(answer, result);
        }
    }
}
