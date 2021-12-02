use std::str::FromStr;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

enum Step {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (dir, value) = s.split_once(' ').ok_or("Wrong step")?;
        let value = value.parse::<i32>().map_err(|err| err.to_string())?;

        Ok(match dir {
            "forward" => Self::Forward(value),
            "down" => Self::Down(value),
            "up" => Self::Up(value),
            _ => unreachable!(),
        })
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<()> {
        let steps: Vec<Step> = parse_lines(input);

        let mut horizontal: i32 = 0;
        let mut depth: i32 = 0;

        for step in steps {
            match step {
                Step::Forward(value) => horizontal += value,
                Step::Down(value) => depth += value,
                Step::Up(value) => depth -= value,
            }
        }

        println!("{}", horizontal * depth);

        Ok(())
    }

    fn part_2(&mut self, input: Option<String>) -> Result<()> {
        let steps: Vec<Step> = parse_lines(input);

        let mut horizontal: i32 = 0;
        let mut depth: i32 = 0;
        let mut aim: i32 = 0;

        for step in steps {
            match step {
                Step::Forward(value) => {
                    horizontal += value;
                    depth += aim * value;
                }
                Step::Down(value) => aim += value,
                Step::Up(value) => aim -= value,
            }
        }

        println!("{}", horizontal * depth);

        Ok(())
    }
}
