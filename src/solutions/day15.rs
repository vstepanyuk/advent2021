use std::fmt::Display;

use pathfinding::prelude::dijkstra;

use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve(&self, matrix: &Matrix<usize>) -> Option<usize> {
        let result = dijkstra(
            &(0, 0),
            |&(x, y)| {
                matrix
                    .neighbours4_iter(x, y)
                    .map(|(&risk, pos)| (pos, risk))
            },
            |pos| *pos == (matrix.width as i32 - 1, matrix.height as i32 - 1),
        );

        result.map(|(_, risk)| risk)
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let matrix = Matrix::<usize>::from(&input).unwrap();

        Ok(Box::new(self.solve(&matrix).unwrap()))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let matrix = Matrix::<usize>::from(&input).unwrap();
        let mut new_matrix = Matrix::new(matrix.width * 5, matrix.height * 5);

        for (i, j, x, y) in itertools::iproduct!(0..5, 0..5, 0..matrix.width, 0..matrix.height) {
            let mut value = *matrix.get(x, y).unwrap() + i + j;
            if value > 9 {
                value -= 9;
            }

            new_matrix.set(i * matrix.width + x, j * matrix.height + y, value);
        }

        Ok(Box::new(self.solve(&new_matrix).unwrap()))
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

        assert_eq!("40", result.to_string());

        let input = include_str!("../../inputs/day15.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();
        assert_eq!("769", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day15_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("315", result.to_string());

        let input = include_str!("../../inputs/day15.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();
        assert_eq!("2963", result.to_string());
    }
}
