use crate::clear;
use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

use std::collections::HashMap;

struct Grid {
    elevations: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: (usize, usize),
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

        let (mut start, mut end) = ((width + 1, height + 1), (width + 1, height + 1));

        let mut elevations: Vec<Vec<u8>> = Vec::new();

        for (y, row) in lines.iter().enumerate() {
            let mut row_elevations: Vec<u8> = Vec::new();

            for (x, mut elevation) in row.trim().chars().enumerate() {
                if elevation == 'S' {
                    start = (x, y);
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
            elevations,
            width,
            height,
            start,
            end,
        })
    }

    fn get_valid_moves(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
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
            if elevation + 1 < offset_elevation {
                continue;
            }

            valid_moves.push(offset_pos);
        }

        valid_moves
    }

    fn get_path_dist_recurse(
        &self,
        pos: (usize, usize),
        path_lens: &mut HashMap<(usize, usize), usize>,
        length: usize,
    ) {
        for valid_move in self.get_valid_moves(pos) {
            if let Some(current_dist) = path_lens.get(&valid_move) {
                if length >= *current_dist {
                    continue;
                }
            }

            path_lens.insert(valid_move, length);

            if valid_move == self.end {
                return;
            }

            self.get_path_dist_recurse(valid_move, path_lens, length + 1);
        }
    }

    fn get_shortest_path(&self) -> usize {
        let mut path_lens = HashMap::new();

        path_lens.insert(self.start, 0);
        self.get_path_dist_recurse(self.start, &mut path_lens, 1);

        path_lens[&self.end]
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let grid = Grid::new(input)?;

    println!("Calculating Shortest Path...");
    let shortest_path = grid.get_shortest_path();
    clear().unwrap();

    Ok((shortest_path.to_string(), "Not Implemented".to_string()))
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
        assert_eq!("Not Implemented", result);
    }
}
