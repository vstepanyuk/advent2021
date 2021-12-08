use std::collections::HashMap;
use std::fmt::Display;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

trait Digit {
    fn to_bin(&self) -> u8;
}

impl Digit for &str {
    fn to_bin(&self) -> u8 {
        ('a'..='g')
            .enumerate()
            .filter(|(_, ch)| self.contains(*ch))
            .fold(0, |result, (index, _)| result | (1 << index))
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
                    .map(|s| s.split(' ').map(|s| s.to_bin()).collect())
                    .collect()
            })
            .collect::<Vec<Vec<Vec<u8>>>>();

        let result = entries
            .iter()
            .map(|entry| {
                let (patterns, output) = (&entry[0], &entry[1]);

                let digit_one = patterns.iter().find(|a| a.count_ones() == 2).unwrap();
                let digit_four = patterns.iter().find(|a| a.count_ones() == 4).unwrap();

                let mapping: HashMap<_, _> = HashMap::from_iter(patterns.iter().map(|&pattern| {
                    (
                        pattern,
                        match pattern.count_ones() {
                            2 => 1,
                            3 => 7,
                            4 => 4,
                            5 if (pattern & digit_one).count_ones() == 2 => 3,
                            5 if (pattern & digit_four).count_ones() == 3 => 5,
                            5 => 2,
                            6 if (pattern & digit_one).count_ones() == 1 => 6,
                            6 if (pattern & digit_four).count_ones() == 4 => 9,
                            6 => 0,
                            7 => 8,
                            _ => unreachable!(),
                        },
                    )
                }));

                output.iter().fold(0, |value, pattern| {
                    value * 10 + *mapping.get(pattern).unwrap()
                })
            })
            .sum::<i32>();

        Ok(Box::new(result))
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
