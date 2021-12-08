use std::fmt::Display;

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve<F>(&self, input: Option<String>, cost: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        let data: Vec<i32> = input
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let min_align = *data.iter().min().unwrap();
        let max_align = *data.iter().max().unwrap();

        (min_align..max_align)
            .map(|align| data.iter().map(|&v| cost(v, align)).sum())
            .min()
            .unwrap()
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, |v, a| (v - a).abs())))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(
            self.solve(input, |v, a| (1..=(v - a).abs()).sum()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day7_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("37", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day7_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("168", result.to_string())
    }
}
