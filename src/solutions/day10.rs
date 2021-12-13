use crate::day10::Route::{Incomplete, Valid};
use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;
use tap::Tap;

#[derive(Default)]
pub struct DaySolution;

enum Route {
    Invalid(char),
    Incomplete(VecDeque<char>),
    Valid,
}

impl FromStr for Route {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut stack: VecDeque<char> = VecDeque::new();

        for ch in s.chars() {
            if ch == '(' || ch == '[' || ch == '<' || ch == '{' {
                stack.push_front(ch);
            } else if let Some(value) = stack.pop_front() {
                let route = match value {
                    '[' if ch != ']' => Route::Invalid(ch),
                    '(' if ch != ')' => Route::Invalid(ch),
                    '<' if ch != '>' => Route::Invalid(ch),
                    '{' if ch != '}' => Route::Invalid(ch),
                    _ => continue,
                };

                return Ok(route);
            } else {
                return Ok(Route::Invalid(ch));
            }
        }

        if !stack.is_empty() {
            return Ok(Incomplete(stack));
        }

        Ok(Valid)
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let result = parse_lines::<Route>(input)
            .iter()
            .map(|route| match route {
                Route::Invalid(ch) => match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                },
                _ => 0,
            })
            .sum::<u64>();

        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let scores = parse_lines::<Route>(input)
            .iter()
            .filter_map(|route| match route {
                Route::Incomplete(stack) => Some(stack),
                _ => None,
            })
            .map(|stack| {
                stack.iter().fold(0, |acc, ch| {
                    acc * 5
                        + match ch {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => 0,
                        }
                })
            })
            .collect::<Vec<u64>>()
            .tap_mut(|v| v.sort_unstable());

        Ok(Box::new(scores[scores.len() / 2]))
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day10_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("26397", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day10_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("288957", result.to_string())
    }
}
