use crate::solutions::{Result, Solution};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        todo!()
    }

    fn part_2(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        todo!()
    }
}
