use crate::helpers;
use crate::solutions::{Result, Solution};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve(&self, arr: &[u32]) -> u32 {
        arr.iter()
            .fold((0, None), |(acc, prev), curr| match prev {
                Some(prev) if prev < curr => (acc + 1, Some(curr)),
                _ => (acc, Some(curr)),
            })
            .0
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let nums = helpers::parse_lines(input);

        Ok(Box::new(self.solve(&nums)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let sums = helpers::parse_lines(input)
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<_>>();

        Ok(Box::new(self.solve(&sums)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day1_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("7", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day1_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("5", result.to_string())
    }
}
