use std::collections::HashMap;
use std::fmt::Display;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

trait Sorted {
    fn resorted(&self) -> String;
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

        for (l1, l2) in entries {
            let mut hash_map: HashMap<String, i32> = HashMap::new();
            let ll1 = l1.clone();

            let one = ll1.iter().find(|a| a.len() == 2).unwrap();
            let four = ll1.iter().find(|a| a.len() == 4).unwrap();

            for d in l1 {
                let digit = match d.len() {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    5 => {
                        if d.chars()
                            .filter(|ch| return one.chars().any(|c| c == *ch))
                            .count()
                            == 2
                        {
                            3
                        } else if d
                            .chars()
                            .filter(|ch| return four.chars().any(|c| c == *ch))
                            .count()
                            == 3
                        {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        // 0, 9 , 6
                        if d.chars()
                            .filter(|ch| return one.chars().any(|c| c == *ch))
                            .count()
                            == 1
                        {
                            6
                        } else if d
                            .chars()
                            .filter(|ch| return four.chars().any(|c| c == *ch))
                            .count()
                            == 4
                        {
                            9
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };

                hash_map.insert(d, digit);
            }

            let mut string = String::default();
            for ll2 in l2.iter() {
                let digit = hash_map.get(ll2).unwrap();
                string = format!("{}{}", string, digit);
            }

            sum += string.parse::<i32>().unwrap();
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
