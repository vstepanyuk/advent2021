use std::error::Error;

use crate::solutions::Solution;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn parse(&self, input: Option<String>) -> Vec<u32> {
        input
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.parse().ok())
            .collect()
    }

    fn solve(&self, arr: &[u32]) -> u32 {
        let (increased, _) = arr.iter().fold((0, None), |acc, curr| match acc {
            (value, Some(prev)) if prev < curr => (value + 1, Some(curr)),
            (value, _) => (value, Some(curr)),
        });

        increased
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<(), Box<dyn Error>> {
        let nums = self.parse(input);
        println!("{}", self.solve(&nums));

        Ok(())
    }

    fn part_2(&mut self, input: Option<String>) -> Result<(), Box<dyn Error>> {
        let nums = self.parse(input);
        let sums: Vec<u32> = nums.windows(3).map(|w| w.iter().sum()).collect();

        println!("{}", self.solve(&sums));

        Ok(())
    }
}
