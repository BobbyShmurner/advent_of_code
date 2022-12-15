use crate::clear;
use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

use std::collections::HashMap;

struct Grid {
    start_points: Vec<(usize, usize)>,
    elevations: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    end: (usize, usize),
}

impl Grid {
    fn new(input: &str) -> Result<Self, BoxedError> {
        if !input.contains('S') || !input.contains('E') {
            return_err!("Input is missing a start or end point!");
        }

        let lines = input.trim().lines().collect::<Vec<&str>>();

        let width = lines[0].len();
        let height = lines.len();

        let mut end = (0, 0);
        let mut elevations: Vec<Vec<u8>> = Vec::new();
        let mut start_points: Vec<(usize, usize)> = Vec::new();

        for (y, row) in lines.iter().enumerate() {
            let mut row_elevations: Vec<u8> = Vec::new();

            for (x, mut elevation) in row.trim().chars().enumerate() {
                if elevation == 'a' {
                    start_points.push((x, y));
                } else if elevation == 'S' {
                    start_points.pop();
                    start_points.insert(0, (x, y));

                    elevation = 'a';
                } else if elevation == 'E' {
                    end = (x, y);
                    elevation = 'z';
                }

                if !('a'..='z').contains(&elevation) {
                    return_err!("Invalid char \'{}\' at position ({}, {})", elevation, x, y);
                }

                row_elevations.push((elevation as u8) - 97);
            }

            elevations.push(row_elevations);
        }

        Ok(Grid {
            start_points,
            elevations,
            width,
            height,
            end,
        })
    }

    fn get_valid_moves(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        const MOVE_OFFSETS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

        let mut valid_moves = Vec::new();
        let elevation = self.elevations[pos.1][pos.0];

        for offset in MOVE_OFFSETS {
            if (pos.0 as i32) + offset.0 < 0
                || (pos.1 as i32) + offset.1 < 0
                || (pos.0 as i32) + offset.0 >= self.width as i32
                || (pos.1 as i32) + offset.1 >= self.height as i32
            {
                continue;
            }

            let offset_pos = (
                (pos.0 as i32 + offset.0) as usize,
                (pos.1 as i32 + offset.1) as usize,
            );

            let offset_elevation = self.elevations[offset_pos.1][offset_pos.0];

            if offset_elevation + 1 < elevation {
                continue;
            }

            valid_moves.push(offset_pos);
        }

        valid_moves
    }

    fn get_shortest_paths(&self) -> (usize, usize) {
        let mut queue = vec![self.end];
        let mut visited = HashMap::new();

        visited.insert(self.end, 0_usize);

        while !queue.is_empty() {
            let pos = queue.pop().unwrap();
            let dist = visited[&pos];

            for valid_move in self.get_valid_moves(&pos) {
                if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(valid_move) {
                    e.insert(dist + 1);
                    queue.insert(0, valid_move);
                }
            }
        }

        let dist_from_start = visited[&self.start_points[0]];
        let mut shortest_dist = usize::MAX;

        for pos in self.start_points.iter().skip(1) {
            if let Some(dist) = visited.get(pos) {
                if dist < &shortest_dist {
                    shortest_dist = *dist;
                }
            }
        }

        (dist_from_start, shortest_dist)
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let grid = Grid::new(input)?;

    let (shortest_from_start, shortest_overall) = grid.get_shortest_paths();
    clear().unwrap();

    Ok((
        shortest_from_start.to_string(),
        shortest_overall.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("31", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("29", result);
    }
}
