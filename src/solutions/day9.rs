use crate::helpers::*;
use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();
        let mut result = 0;

        for x in 0..matrix.width {
            for y in 0..matrix.height {
                let value = matrix.get(x, y).unwrap();
                if matrix.neighbours(x, y).iter().all(|&v| v > value) {
                    result += value + 1;
                }
            }
        }

        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);
        let width = lines[0].len();

        let mut heightmap = lines
            .iter()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()))
            .flatten()
            .collect::<Vec<_>>();

        let mut counts: Vec<i32> = vec![];
        let mut queue: VecDeque<_> = VecDeque::new();

        for i in 0..heightmap.len() {
            if heightmap[i] >= 9 {
                continue;
            }

            queue.push_back(i);

            let mut count = 0;
            while let Some(i) = queue.pop_front() {
                if heightmap[i] == 9 {
                    continue;
                }

                heightmap[i] = 9;
                count += 1;

                queue.extend(
                    heightmap
                        .neighbour_indexes(i, width)
                        .into_iter()
                        .filter(|&i| heightmap[i] < 9),
                );
            }

            if count > 0 {
                counts.push(count);
            }
        }

        counts.sort_unstable();

        Ok(Box::new(counts.iter().rev().take(3).product::<i32>()))
    }
}

#[cfg(test)]
mod tests {
    use crate::day9::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day9_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("15", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day9_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("1134", result.to_string())
    }
}
