use crate::macros::*;
use crate::DayReturnType;

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// #[derive(Clone, Copy, PartialEq)]
// struct Pos {
//     x: i32,
//     y: i32,
// }

// impl Pos {
//     fn new(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }

//     fn zero() -> Self {
//         Self { x: 0, y: 0 }
//     }

//     fn direction(dir: &Direction) -> Self {
//         match *dir {
//             Direction::Up => Self::new(0, 1),
//             Direction::Down => Self::new(0, -1),
//             Direction::Left => Self::new(-1, 0),
//             Direction::Right => Self::new(1, 0),
//         }
//     }

//     fn squared_distance(&self, other: &Self) -> i32 {
//         if self == other {
//             return 0;
//         }

//         (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
//     }
// }

// impl std::ops::Add<Pos> for Pos {
//     type Output = Pos;

//     fn add(self, rhs: Pos) -> Self::Output {
//         Pos {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// impl std::ops::AddAssign<Pos> for Pos {
//     fn add_assign(&mut self, rhs: Pos) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//     }
// }

// impl std::ops::Mul<i32> for Pos {
//     type Output = Pos;

//     fn mul(self, rhs: i32) -> Self::Output {
//         Pos {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }

// struct Rope {
//     positions: Vec<Pos>,
//     unique_tail_positions: Vec<Pos>,
// }

// impl Rope {
//     fn new(length: usize) -> Result<Self, String> {
//         if length < 2 {
//             return Err(format!(
//                 "Cannot create a rope of size {}, ropes must have a length of at least 2!",
//                 length
//             ));
//         }

//         Ok(Self {
//             positions: vec![Pos::zero(); length],
//             unique_tail_positions: vec![Pos::zero()],
//         })
//     }

//     fn update_tail(&mut self) {
//         if self.head.squared_distance(&self.tail) < 4 {
//             return;
//         }

//         self.tail = self.prev_head;
//         if !self.unique_tail_positions.contains(&self.tail) {
//             self.unique_tail_positions.push(self.tail);
//         }
//     }

//     fn move_head(&mut self, direction: Direction, step_size: i32) {
//         let step = Pos::direction(&direction);

//         for _i in 0..step_size {
//             self.prev_head = self.head;
//             self.head += step;

//             self.update_tail();
//         }
//     }

//     fn move_using_str(&mut self, line: &str) -> Result<(), String> {
//         let (lhs, rhs) = line.trim().split_at(1);

//         let direction = match lhs.to_uppercase().trim() {
//             "U" => Direction::Up,
//             "D" => Direction::Down,
//             "L" => Direction::Left,
//             "R" => Direction::Right,
//             _ => return Err(format!("Invalid Direction \"{}\"", lhs.trim())),
//         };

//         let step_size: i32 = match rhs.trim().parse() {
//             Ok(val) => val,
//             Err(_) => return Err(format!("Invalid Step Size \"{}\"", rhs.trim())),
//         };

//         self.move_head(direction, step_size);

//         Ok(())
//     }
// }

// pub fn execute(input: &str) -> DayReturnType {
//     let mut knot_part1 = match Rope::new(2) {
//         Ok(val) => val,
//         Err(e) => return Err(Box::new(SimpleError::new(e))),
//     };

//     let mut knot_part2 = match Rope::new(10) {
//         Ok(val) => val,
//         Err(e) => return Err(Box::new(SimpleError::new(e))),
//     };

//     for line in input.trim().lines() {
//         if let Err(e) = knot_part1.move_using_str(line.trim()) {
//             return Err(Box::new(SimpleError::new(e)));
//         }

//         if let Err(e) = knot_part2.move_using_str(line.trim()) {
//             return Err(Box::new(SimpleError::new(e)));
//         }
//     }

//     Ok((
//         knot_part1.unique_tail_positions.len().to_string(),
//         knot_part2.unique_tail_positions.len().to_string(),
//     ))
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn part1_example() {
//         let input = r#"R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2"#;

//         let result = super::execute(input).unwrap().0;
//         assert_eq!("13", result);
//     }

//     #[test]
//     fn part2_example() {
//         let input = r#"R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2"#;

//         let result = super::execute(input).unwrap().1;
//         assert_eq!("Answer2", result);
//     }
// }

pub fn execute(input: &str) -> DayReturnType {
    return_err!("Code For This Day Is Not Complete!")
}
