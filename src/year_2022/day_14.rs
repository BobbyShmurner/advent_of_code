use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(input: &str) -> Result<Self, BoxedError> {
        let split = input.trim().split(',').collect::<Vec<&str>>();

        if split.len() != 2 {
            return_err!("Invalid co-ordinate \"{}\"", input);
        }

        let x = unwrap_or_return!(
            split[0].trim().parse(),
            "Invalid x co-ordinate \"{}\"",
            split[0].trim()
        );
        let y = unwrap_or_return!(
            split[1].trim().parse(),
            "Invalid y co-ordinate \"{}\"",
            split[1].trim()
        );

        Ok(Self { x, y })
    }

    fn singnum(&self) -> Point {
        Point::new(self.x.signum(), self.y.signum())
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Line {
    points: Vec<Point>,
    target_point: usize,
    point_on_line: Point,
}

impl Line {
    fn new(points: Vec<Point>) -> Result<Self, BoxedError> {
        if points.is_empty() {
            return_err!("Line must have at least 1 point");
        }

        let first_point = points[0];
        let step = (points[1] - first_point).singnum();

        Ok(Line {
            points,
            target_point: 1,
            point_on_line: first_point - step,
        })
    }

    fn parse(line: &str) -> Result<Self, BoxedError> {
        let mut points = Vec::new();

        for point in line.trim().split(" -> ") {
            points.push(Point::parse(point)?);
        }

        Line::new(points)
    }
}

impl std::iter::Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.target_point >= self.points.len() {
            return None;
        }

        let target = self.points[self.target_point];
        let step = (target - self.point_on_line).singnum();

        self.point_on_line += step;

        if self.point_on_line == target {
            self.target_point += 1;
        }

        Some(self.point_on_line)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.points[0].to_string())?;

        for point in self.points.iter().skip(1) {
            f.write_fmt(format_args!(" -> {point}"))?;
        }

        Ok(())
    }
}

enum Tile {
    Rock,
    Sand,
}

struct Grid {
    points: HashMap<Point, Tile>,
    resting: usize,
    min: Point,
    max: Point,
}

impl Grid {
    fn new(points: Vec<Point>) -> Grid {
        let mut min = Point::new(500, 0);
        let mut max = Point::new(500, 0);

        let mut points_map = HashMap::<Point, Tile>::new();

        for point in points.iter() {
            if point.x < min.x {
                min.x = point.x;
            }

            if point.x > max.x {
                max.x = point.x;
            }

            if point.y < min.y {
                min.y = point.y;
            }

            if point.y > max.y {
                max.y = point.y;
            }

            if !points_map.contains_key(point) {
                points_map.insert(*point, Tile::Rock);
            }
        }

        max.y += 2;

        Grid {
            points: points_map,
            min,
            resting: 0,
            max,
        }
    }

    fn parse(input: &str) -> Result<Self, BoxedError> {
        let mut points = Vec::new();

        for line in input.trim().lines() {
            let line = Line::parse(line)?;
            points.extend(line);
        }

        Ok(Grid::new(points))
    }

    fn try_step_point(&mut self, point: &mut Point, step: Point) -> bool {
        let new_point = *point + step;

        if self.points.contains_key(&new_point) || new_point.y == self.max.y {
            return false;
        }

        *point = new_point;
        true
    }

    /// Returns `true` if sand falls off into the abyss
    fn add_sand(&mut self) -> (bool, bool) {
        let mut point = Point::new(500, 0);
        let mut hit_floor = false;

        loop {
            if point.y > self.max.y - 2 {
                hit_floor = true;
            }

            if self.try_step_point(&mut point, Point::new(0, 1)) {
                continue;
            }

            if self.try_step_point(&mut point, Point::new(-1, 1)) {
                continue;
            }

            if self.try_step_point(&mut point, Point::new(1, 1)) {
                continue;
            }

            break;
        }

        self.points.insert(point, Tile::Sand);
        self.resting += 1;

        (hit_floor, point.y == 0)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y - 10..=self.max.y + 10 {
            f.write_str("\n")?;

            for x in self.min.x - 10..=self.max.x + 10 {
                let point = Point::new(x, y);

                if point.x == 500 && point.y == 0 {
                    f.write_str("+")?;
                } else if let Some(tile) = self.points.get(&point) {
                    match tile {
                        Tile::Rock => f.write_str("#")?,
                        Tile::Sand => f.write_str("o")?,
                    };
                } else if point.y >= self.max.y {
                    f.write_str("#")?;
                } else {
                    f.write_str(" ")?;
                }
            }
        }

        Ok(())
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let mut grid = Grid::parse(input)?;
    let mut hit_floor_at = 0;

    loop {
        let (hit_floor, finished) = grid.add_sand();

        if hit_floor && hit_floor_at == 0 {
            hit_floor_at = grid.resting - 1;
        }

        if finished {
            break;
        }
    }

    Ok((hit_floor_at.to_string(), grid.resting.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("24", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("93", result);
    }
}
