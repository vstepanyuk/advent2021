use crate::solutions::{Result, Solution};
use std::cmp::min;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let data: Vec<i32> = input
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut min_sum = i32::MAX;
        let min_fuel = *data.iter().min().unwrap();
        let max_fuel = *data.iter().max().unwrap();

        for i in min_fuel..max_fuel {
            let sum: i32 = data.iter().map(|v| (v - i).abs()).sum();
            min_sum = min(sum, min_sum);
        }

        Ok(Box::new(min_sum))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let data: Vec<i32> = input
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut min_sum = i32::MAX;
        let min_fuel = *data.iter().min().unwrap();
        let max_fuel = *data.iter().max().unwrap();

        for i in min_fuel..max_fuel {
            let sum: i32 = data
                .iter()
                .map(|v| {
                    let mut cost = 0;
                    for j in 0..=(v - i).abs() {
                        cost += j;
                    }

                    cost
                })
                .sum();
            min_sum = min(sum, min_sum);
        }

        Ok(Box::new(min_sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day7_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("37", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day7_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("", result.to_string())
    }
}
