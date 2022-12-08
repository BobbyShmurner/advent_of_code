use std::ops::Range;

use simple_error::SimpleError;

use crate::DayReturnType;

struct Trees {
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Trees {
    fn new(lines: Vec<&str>) -> Result<Trees, String> {
        let width = lines[0].trim().len();
        let height = lines.len();

        let mut data = Vec::new();

        for line in lines {
            let mut line_heights = Vec::new();

            for height_char in line.trim().chars() {
                let height = match height_char.to_digit(10) {
                    Some(val) => val,
                    None => {
                        return Err(format!(
                            "Failed to parse input! \"{}\" is not a valid height for a tree",
                            height_char
                        ))
                    }
                };

                line_heights.push(height);
            }

            data.push(line_heights);
        }

        Ok(Trees {
            data,
            width,
            height,
        })
    }

    fn get_tree_height(&self, x: usize, y: usize) -> u32 {
        self.data[y][x]
    }

    fn get_scenic_score_and_outside_visability_in_range(
        &self,
        x: usize,
        y: usize,
        range: Range<usize>,
        is_row: bool,
        reverse_range: bool,
    ) -> (u32, bool) {
        let height = self.get_tree_height(x, y);
        let mut score = 0;

        let range: Vec<usize> = if reverse_range {
            range.rev().collect()
        } else {
            range.collect()
        };

        for i in range {
            score += 1;

            if is_row {
                if self.get_tree_height(i, y) >= height {
                    return (score, false);
                }
            } else if self.get_tree_height(x, i) >= height {
                return (score, false);
            }
        }

        (score, true)
    }

    fn get_scenic_score_and_outside_visability(&self, x: usize, y: usize) -> (u32, bool) {
        // If On Edge
        if x < 1 || y < 1 || x > self.width - 2 || y > self.height - 2 {
            return (0, true);
        }

        let (left_score, visable_from_left) =
            self.get_scenic_score_and_outside_visability_in_range(x, y, 0..x, true, true);
        let (right_score, visable_from_right) = self
            .get_scenic_score_and_outside_visability_in_range(x, y, x + 1..self.width, true, false);
        let (up_score, visable_from_up) =
            self.get_scenic_score_and_outside_visability_in_range(x, y, 0..y, false, true);
        let (down_score, visable_from_down) = self
            .get_scenic_score_and_outside_visability_in_range(
                x,
                y,
                y + 1..self.height,
                false,
                false,
            );

        let visable_from_outside =
            visable_from_left || visable_from_right || visable_from_up || visable_from_down;

        let scenic_score = left_score * right_score * up_score * down_score;

        (scenic_score, visable_from_outside)
    }

    fn get_heightest_scenic_score_and_count_visable_trees(&self) -> (u32, u32) {
        let mut visable_count = 0;
        let mut highest_scenic_score = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                let (scenic_score, is_visable) = self.get_scenic_score_and_outside_visability(x, y);

                if scenic_score > highest_scenic_score {
                    highest_scenic_score = scenic_score;
                }

                if is_visable {
                    visable_count += 1;
                }
            }
        }

        (visable_count, highest_scenic_score)
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Err(Box::new(SimpleError::new(
            "Input must have at least one line!",
        )));
    }

    let trees = match Trees::new(lines) {
        Ok(val) => val,
        Err(e) => return Err(Box::new(SimpleError::new(e))),
    };

    let (visable_count, highest_scenic_score) =
        trees.get_heightest_scenic_score_and_count_visable_trees();

    Ok((visable_count.to_string(), highest_scenic_score.to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"30373
25512
65332
33549
35390"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("21", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"30373
25512
65332
33549
35390"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("8", result);
    }
}
