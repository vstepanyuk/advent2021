use std::error::Error;

use crate::solutions::Solution;

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, _input: Option<String>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn part_2(&mut self, _input: Option<String>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
