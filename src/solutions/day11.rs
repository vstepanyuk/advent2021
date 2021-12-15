use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use crate::matrix::{Matrix, MATRIX_NEIGHBOURS_8};
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

        let mut flashed = HashSet::new();
        while let Some((x, y)) = queue.pop_front() {
            if flashed.contains(&(x, y)) {
                continue;
            }

            flashed.insert((x, y));
            queue.extend(MATRIX_NEIGHBOURS_8.iter().filter_map(|(dx, dy)| {
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
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();
        let result = (0..100).map(|_| self.step(&mut matrix)).sum::<usize>();

        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();

        let result = std::iter::repeat(1)
            .take_while(|_| self.step(&mut matrix) != matrix.size())
            .sum::<usize>();

        Ok(Box::new(result + 1))
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
