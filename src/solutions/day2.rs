use std::fmt::Display;
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
        let (dir, value) = s.split_once(' ').ok_or("wrong step")?;
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

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
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

        Ok(Box::new(horizontal * depth))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
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

        Ok(Box::new(horizontal * depth))
    }
}

// Bash solution
// echo "Part 1: $(cat input.txt | sed -r 's/forward (.*)/\1 0/g' | sed -r 's/down /0 /g' | sed -r 's/up /0 -/g' | awk '{ h += $1; d += $2 } END { print d * h; }')"
// echo "Part 2: $(cat input.txt | sed -r 's/forward (.*)/\1 \1 0 1/g' | sed -r 's/down (.*)/0 0 \1 0/g' | sed -r 's/up (.*)/0 0 -\1 0/g' | awk '{ h += $1; a +=$3; d += $2 * a * $4} END { print d * h}')"
