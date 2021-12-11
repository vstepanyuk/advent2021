use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution {}

impl DaySolution {
    fn step(&self, matrix: &mut Matrix<i32>) -> usize {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        queue.extend(matrix.iter_mut().filter_map(|(energy, (x, y))| {
            *energy += 1;
            (*energy > 9).then(|| (x as i32, y as i32))
        }));

        let offsets = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        let mut flashed = HashSet::new();
        while let Some((x, y)) = queue.pop_front() {
            if flashed.contains(&(x, y)) {
                continue;
            }

            flashed.insert((x, y));
            queue.extend(offsets.iter().filter_map(|(dx, dy)| {
                matrix.get_mut(x + dx, y + dy).and_then(|energy| {
                    *energy += 1;
                    (*energy > 9).then(|| (x + dx, y + dy))
                })
            }));
        }

        matrix.iter_mut().for_each(|(energy, _)| {
            if *energy > 9 {
                *energy = 0;
            }
        });

        flashed.len()
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();

        let mut c = 0;
        for _ in 0..100 {
            c += self.step(&mut matrix);
        }

        Ok(Box::new(c))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();

        let mut c = 0;
        while self.step(&mut matrix) != matrix.width * matrix.height {
            c += 1;
        }
        Ok(Box::new(c + 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day11_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("1656", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day11_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("195", result.to_string())
    }
}
