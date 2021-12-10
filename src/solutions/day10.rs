use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn validate(&self, s: &str) -> Option<char> {
        let mut stack: VecDeque<char> = VecDeque::new();

        let is_debug = s == "{([(<{}[<>[]}>{[]{[(<()>";

        for ch in s.chars() {
            // if is_debug {
            // println!("{:?}", stack);
            // }

            if ch == '(' || ch == '[' || ch == '<' || ch == '{' {
                stack.push_front(ch);
            } else {
                if let Some(value) = stack.pop_front() {
                    // if is_debug {
                    // println!("{} = {}", value, ch);
                    // }

                    match value {
                        '[' if ch != ']' => return Some(ch),
                        '(' if ch != ')' => return Some(ch),
                        '<' if ch != '>' => return Some(ch),
                        '{' if ch != '}' => return Some(ch),
                        _ => continue,
                    }
                } else {
                    return Some(ch);
                }
            }
        }

        None
    }

    fn complete(&self, s: &str) -> VecDeque<char> {
        let mut stack: VecDeque<char> = VecDeque::new();

        // let is_debug = s == "{([(<{}[<>[]}>{[]{[(<()>";

        for ch in s.chars() {
            // if is_debug {
            // println!("{:?}", stack);
            // }

            if ch == '(' || ch == '[' || ch == '<' || ch == '{' {
                stack.push_front(ch);
            } else {
                stack.pop_front();
            }
        }

        stack
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines: Vec<String> = parse_lines(input);

        let mut result = 0;

        for line in lines {
            println!("{}", line);
            if let Some(ch) = self.validate(&line) {
                result += match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
            }
        }

        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines: Vec<String> = parse_lines(input);

        let mut scores = vec![];
        for line in lines {
            if let Some(ch) = self.validate(&line) {
                continue;
            }

            let mut result: i64 = 0;
            let mut a = self.complete(&line);
            while !a.is_empty() {
                let ch = a.pop_front().unwrap();
                result = result * 5
                    + match ch {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
            }
            scores.push(result);
        }

        scores.sort_unstable();
        Ok(Box::new(scores[scores.len() / 2]))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day10::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day10_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day10_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
