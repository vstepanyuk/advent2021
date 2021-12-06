use crate::solutions::{Result, Solution};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve(&self, input: Option<String>, days: usize) -> u64 {
        let fish: Vec<i32> = input
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut counter = (0..9)
            .map(|i| fish.iter().filter(|&&f| f == i).count() as u64)
            .collect::<Vec<u64>>();

        for _ in 0..days {
            counter.rotate_left(1);
            counter[6] += counter[8];
        }

        counter.iter().sum()
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 80)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 256)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day6_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("5934", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day6_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("26984457539", result.to_string())
    }
}
