use std::collections::VecDeque;
use std::fmt::Display;

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let blocks = input
            .unwrap()
            .split("inp w\n")
            .skip(1)
            .map(|s| s.lines().map(String::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut model = [0; 14];
        let mut stack = VecDeque::new();
        for (index, block) in blocks.iter().enumerate() {
            if block[3] == "div z 1" {
                let y = block[14].replace("add y ", "").parse::<i32>().unwrap();
                stack.push_front((index, y));
            } else {
                let (model_num_position, y) = stack.pop_front().unwrap();
                let x = block[4].replace("add x ", "").parse::<i32>().unwrap();
                if x + y > 0 {
                    model[index] = 9;
                    model[model_num_position] = 9 - x - y;
                } else {
                    model[index] = 9 + x + y;
                    model[model_num_position] = 9;
                }
            }
        }

        Ok(Box::new(model.map(|v| v.to_string()).join("")))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let blocks = input
            .unwrap()
            .split("inp w\n")
            .skip(1)
            .map(|s| s.lines().map(String::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut model = [0; 14];
        let mut stack = VecDeque::new();
        for (index, block) in blocks.iter().enumerate() {
            if block[3] == "div z 1" {
                let y = block[14].replace("add y ", "").parse::<i32>().unwrap();
                stack.push_front((index, y));
            } else {
                let (model_num_position, y) = stack.pop_front().unwrap();
                let x = block[4].replace("add x ", "").parse::<i32>().unwrap();

                if x + y > 0 {
                    model[index] = 1 + x + y;
                    model[model_num_position] = 1;
                } else {
                    model[index] = 1;
                    model[model_num_position] = 1 - x - y;
                }
            }
        }

        Ok(Box::new(model.map(|v| v.to_string()).join("")))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day24::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day24_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day24_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
