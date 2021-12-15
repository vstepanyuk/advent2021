use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve(&self, matrix: &Matrix<i32>) -> i32 {
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), (0, 0)));

        let offsets = [(1, 0), (0, 1), (0, -1), (-1, 0)];
        let mut visited = HashSet::<(i32, i32)>::new();

        let mut min_risk = 0;
        while let Some((Reverse(risk), (x, y))) = q.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            if x == matrix.width as i32 - 1 && y == matrix.height as i32 - 1 {
                min_risk = risk;
                break;
            }

            for (dx, dy) in offsets {
                if let Some(&value) = matrix.get(x + dx, y + dy) {
                    if visited.contains(&(x + dx, y + dy)) {
                        continue;
                    }
                    q.push((Reverse(value + risk), (x + dx, y + dy)))
                }
            }
        }

        min_risk
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let matrix = Matrix::<i32>::from(&input).unwrap();

        Ok(Box::new(self.solve(&matrix)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let matrix = Matrix::<i32>::from(&input).unwrap();
        let mut new_matrix = Matrix::new(matrix.width * 5, matrix.height * 5);

        for j in 0..5 {
            for i in 0..5 {
                for y in 0..matrix.width {
                    for x in 0..matrix.height {
                        let mut value = *matrix.get(x, y).unwrap() + (i + j) as i32;
                        if value > 9 {
                            value -= 9;
                        }

                        new_matrix.set(i * matrix.width + x, j * matrix.height + y, value);
                    }
                }
            }
        }

        Ok(Box::new(self.solve(&new_matrix)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day15_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("40", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day15_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("315", result.to_string())
    }
}
