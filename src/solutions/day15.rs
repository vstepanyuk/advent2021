use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve(&self, matrix: &Matrix<usize>) -> usize {
        let mut priority_queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        priority_queue.push((Reverse(0), (0, 0)));
        let mut min_risk = 0;
        while let Some((Reverse(risk), (x, y))) = priority_queue.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            if (x, y) == (matrix.width as i32 - 1, matrix.height as i32 - 1) {
                min_risk = risk;
                break;
            }

            priority_queue.extend(
                matrix
                    .neighbours4_iter(x, y)
                    .filter(|(_, pos)| !visited.contains(pos))
                    .map(|(value, pos)| (Reverse(value + risk), pos)),
            );
        }

        min_risk
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let matrix = Matrix::<usize>::from(&input).unwrap();

        Ok(Box::new(self.solve(&matrix)))
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
