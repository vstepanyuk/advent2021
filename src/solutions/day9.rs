use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

trait VecMap<T: Clone> {
    fn neighbour_indexes(&self, index: usize, width: usize) -> Vec<usize>;
}

impl<T> VecMap<T> for Vec<T>
where
    T: Clone,
{
    fn neighbour_indexes(&self, index: usize, width: usize) -> Vec<usize> {
        let x = index % width;
        let y = index / width;

        let mut result = vec![];
        if x > 0 {
            result.push(x - 1 + y * width);
        }

        if y > 0 {
            result.push(x + (y - 1) * width);
        }

        if (x + 1) < width {
            result.push(x + 1 + y * width);
        }

        if (x + (y + 1) * width) < self.len() {
            result.push(x + (y + 1) * width);
        }

        result
    }
}

impl DaySolution {}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);
        let width = lines[0].len();

        let heightmap = lines
            .iter()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()))
            .flatten()
            .collect::<Vec<_>>();

        let result = heightmap
            .iter()
            .enumerate()
            .filter(|(index, &value)| {
                heightmap
                    .neighbour_indexes(*index, width)
                    .iter()
                    .all(|&n_index| value < heightmap[n_index])
            })
            .map(|(_, &value)| value + 1)
            .sum::<u32>();

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
                let value = heightmap[i];
                if value == 9 {
                    continue;
                }

                heightmap[i] = 9;
                count += 1;

                heightmap.neighbour_indexes(i, width).iter().for_each(|&i| {
                    queue.push_back(i);
                })
            }

            counts.push(count);
        }

        counts.sort();

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
