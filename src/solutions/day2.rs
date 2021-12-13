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

impl DaySolution {
    fn solve(&self, input: Option<String>) -> (i32, i32, i32) {
        parse_lines::<Step>(input)
            .iter()
            .fold((0, 0, 0), |(horizontal, depth, aim), step| match step {
                Step::Forward(value) => (horizontal + value, depth + aim * value, aim),
                Step::Down(value) => (horizontal, depth, aim + value),
                Step::Up(value) => (horizontal, depth, aim - value),
            })
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (horizontal, _, depth) = self.solve(input);
        Ok(Box::new(horizontal * depth))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (horizontal, depth, _) = self.solve(input);
        Ok(Box::new(horizontal * depth))
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day2_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("150", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day2_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("900", result.to_string())
    }
}

// Bash solution
// echo "Part 1: $(cat input.txt | sed -r 's/forward (.*)/\1 0/g' | sed -r 's/down /0 /g' | sed -r 's/up /0 -/g' | awk '{ h += $1; d += $2 } END { print d * h; }')"
// echo "Part 2: $(cat input.txt | sed -r 's/forward (.*)/\1 \1 0 1/g' | sed -r 's/down (.*)/0 0 \1 0/g' | sed -r 's/up (.*)/0 0 -\1 0/g' | awk '{ h += $1; a +=$3; d += $2 * a * $4} END { print d * h}')"
