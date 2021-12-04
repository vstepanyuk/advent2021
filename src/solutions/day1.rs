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
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let nums = helpers::parse_lines::<u32>(input);

        Ok(Box::new(self.solve(&nums)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let sums: Vec<u32> = helpers::parse_lines::<u32>(input)
            .windows(3)
            .map(|w| w.iter().sum())
            .collect();

        Ok(Box::new(self.solve(&sums)))
    }
}
