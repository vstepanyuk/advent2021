use crate::helpers::parse_lines;
use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut count = 0;

        let mut pos: Vec<(usize, usize)> = vec![];
        let mut width = 0;
        let mut height = 0;
        let mut input = input.unwrap();

        for line in input.lines() {
            count += 1;
            if line == "" {
                break;
            }
            let (x, y) = line.split_once(",").unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
            if x > width {
                width = x;
            }

            if y > height {
                height = y;
            }

            pos.push((x, y));
        }

        let mut flips = vec![];
        for line in input.lines().skip(count) {
            if line.starts_with("fold along y=") {
                let value = line
                    .strip_prefix("fold along y=")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                flips.push((value, "y"));
            } else {
                let value = line
                    .strip_prefix("fold along x=")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                flips.push((value, "x"));
            }
        }

        // println!("{:?}", flips);

        let mut matrix = Matrix::new(width + 1, height + 1);
        for p in pos {
            matrix.set(p.0, p.1, 1);
        }

        // println!("{:?}", matrix);

        for flip in flips {
            let (width, height) = if flip.1 == "y" {
                (matrix.width, matrix.height / 2)
            } else {
                (matrix.width / 2, matrix.height)
            };

            let mut new_matrix = Matrix::new(width, height);
            if flip.1 == "y" {
                for y in 0..height {
                    for x in 0..width {
                        let value1 = *matrix.get(x, y).unwrap();
                        new_matrix.set(x, y, value1);
                        let value2 = *matrix.get(x, matrix.height - y - 1).unwrap();

                        new_matrix.set(x, y, value1 | value2);
                    }
                }
            }
            if flip.1 == "x" {
                for y in 0..height {
                    for x in 0..width {
                        let value1 = *matrix.get(x, y).unwrap();
                        new_matrix.set(x, y, value1);
                        let value2 = *matrix.get(matrix.width - x - 1, y).unwrap();

                        new_matrix.set(x, y, value1 | value2);
                    }
                }
            }

            count = new_matrix.iter().filter(|(v, _)| **v == 1).count();

            break;
        }

        Ok(Box::new(count))
        // let mut matrix = Matrix::new()
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut count = 0;

        let mut pos: Vec<(usize, usize)> = vec![];
        let mut width = 0;
        let mut height = 0;
        let mut input = input.unwrap();

        for line in input.lines() {
            count += 1;
            if line == "" {
                break;
            }
            let (x, y) = line.split_once(",").unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
            if x > width {
                width = x;
            }

            if y > height {
                height = y;
            }

            pos.push((x, y));
        }

        let mut flips = vec![];
        for line in input.lines().skip(count) {
            if line.starts_with("fold along y=") {
                let value = line
                    .strip_prefix("fold along y=")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                flips.push((value, "y"));
            } else {
                let value = line
                    .strip_prefix("fold along x=")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                flips.push((value, "x"));
            }
        }

        // println!("{:?}", flips);

        let mut matrix = Matrix::new(width + 1, height + 1);
        for p in pos {
            matrix.set(p.0, p.1, 1);
        }

        // println!("{:?}", matrix);

        for flip in flips {
            let (width, height) = if flip.1 == "y" {
                (matrix.width, matrix.height / 2)
            } else {
                (matrix.width / 2, matrix.height)
            };

            let mut new_matrix = Matrix::new(width, height);
            if flip.1 == "y" {
                for y in 0..height {
                    for x in 0..width {
                        let value1 = *matrix.get(x, y).unwrap();
                        new_matrix.set(x, y, value1);
                        let value2 = *matrix.get(x, matrix.height - y - 1).unwrap();

                        new_matrix.set(x, y, value1 | value2);
                    }
                }
            }
            if flip.1 == "x" {
                for y in 0..height {
                    for x in 0..width {
                        let value1 = *matrix.get(x, y).unwrap();
                        new_matrix.set(x, y, value1);
                        let value2 = *matrix.get(matrix.width - x - 1, y).unwrap();

                        new_matrix.set(x, y, value1 | value2);
                    }
                }
            }

            matrix = new_matrix;
        }

        Ok(Box::new(format!("{:?}", matrix)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day13_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("17", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day13_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("", result.to_string())
    }
}
