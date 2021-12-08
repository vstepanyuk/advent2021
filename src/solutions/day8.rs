use std::collections::HashMap;
use std::fmt::Display;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

trait Sorted {
    fn resorted(&self) -> String;
}

trait Substring {
    fn count_chars_from(&self, from: &Self) -> usize;
}

impl Substring for String {
    fn count_chars_from(&self, from: &Self) -> usize {
        self.chars()
            .filter(|ch| from.chars().any(|c| c == *ch))
            .count()
    }
}

impl Sorted for &str {
    fn resorted(&self) -> String {
        let mut chars = self.chars().collect::<Vec<char>>();
        chars.sort();
        chars.into_iter().collect()
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lengths_arr = parse_lines::<String>(input)
            .iter()
            .map(|s| {
                let (_, output) = s.split_once(" | ").unwrap();
                output.split(' ').map(|s| s.len()).collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let count = lengths_arr
            .iter()
            .map(|l| l.into_iter().filter(|&a| [2, 3, 4, 7].contains(a)).count())
            .sum::<usize>();

        Ok(Box::new(count))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let entries = parse_lines::<String>(input)
            .iter()
            .map(|s| {
                let (s1, s2) = s.split_once(" | ").unwrap();
                (
                    s1.split(' ').map(|s| s.resorted()).collect::<Vec<String>>(),
                    s2.split(' ').map(|s| s.resorted()).collect::<Vec<String>>(),
                )
            })
            .collect::<Vec<(Vec<String>, Vec<String>)>>();
        let mut sum = 0;

        for (patterns, output) in entries {
            let mut mapping: HashMap<String, i32> = HashMap::new();

            let digit_one = patterns.clone().into_iter().find(|a| a.len() == 2).unwrap();
            let digit_four = patterns.clone().into_iter().find(|a| a.len() == 4).unwrap();

            for pattern in patterns {
                let digit = match pattern.len() {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    5 if pattern.count_chars_from(&digit_one) == 2 => 3,
                    5 if pattern.count_chars_from(&digit_four) == 3 => 5,
                    5 => 2,
                    6 if pattern.count_chars_from(&digit_one) == 1 => 6,
                    6 if pattern.count_chars_from(&digit_four) == 4 => 9,
                    6 => 0,
                    _ => unreachable!(),
                };

                mapping.insert(pattern, digit);
            }

            let result = output.iter().fold(0, |value, pattern| {
                let digit = *mapping.get(pattern).unwrap();
                value * 10 + digit
            });

            sum += result;
        }

        Ok(Box::new(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day8_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("26", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day8_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("61229", result.to_string())
    }
}
