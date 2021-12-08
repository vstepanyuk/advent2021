use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

trait Sorted {
    fn resorted(&self) -> String;
}

trait Substring {
    fn intersection_chars_count(&self, from: &Self) -> usize;
}

impl Sorted for &str {
    fn resorted(&self) -> String {
        let mut chars = self.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        chars.into_iter().collect()
    }
}

impl Substring for String {
    fn intersection_chars_count(&self, from: &Self) -> usize {
        let a: HashSet<_> = HashSet::from_iter(self.chars());
        let b: HashSet<_> = HashSet::from_iter(from.chars());
        a.intersection(&b).count()
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let result = parse_lines::<String>(input)
            .iter()
            .map(|s| {
                s.split_once(" | ")
                    .unwrap()
                    .1
                    .split(' ')
                    .map(|s| s.len())
                    .filter(|l| [2, 3, 4, 7].contains(l))
                    .count()
            })
            .sum::<usize>();

        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let entries = parse_lines::<String>(input)
            .iter()
            .map(|s| {
                s.split(" | ")
                    .map(|s| s.split(' ').map(|s| s.resorted()).collect::<Vec<String>>())
                    .collect::<Vec<Vec<String>>>()
            })
            .collect::<Vec<Vec<Vec<String>>>>();
        let mut sum = 0;

        for entry in entries {
            let (patterns, output) = (&entry[0], &entry[1]);
            let mut mapping: HashMap<String, i32> = HashMap::new();

            let digit_one = patterns.iter().find(|a| a.len() == 2).unwrap();
            let digit_four = patterns.iter().find(|a| a.len() == 4).unwrap();

            for pattern in patterns {
                let digit = match pattern.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 if pattern.intersection_chars_count(digit_one) == 2 => 3,
                    5 if pattern.intersection_chars_count(digit_four) == 3 => 5,
                    5 => 2,
                    6 if pattern.intersection_chars_count(digit_one) == 1 => 6,
                    6 if pattern.intersection_chars_count(digit_four) == 4 => 9,
                    6 => 0,
                    7 => 8,
                    _ => unreachable!(),
                };

                mapping.insert(pattern.to_string(), digit);
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
