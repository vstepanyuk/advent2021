use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let digits = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
        let lines = parse_lines::<String>(input)
            .iter()
            .map(|s| {
                let (s1, s2) = s.split_once(" | ").unwrap();
                s2.split(' ').map(|s| s.len()).collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let mut count = 0;

        for l in lines {
            let c = l
                .into_iter()
                .filter(|a| *a == 2 || *a == 4 || *a == 3 || *a == 7)
                .count();
            // .collect::<Vec<usize>>();

            count += c;
        }

        Ok(Box::new(count))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let digits = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

        let lines = parse_lines::<String>(input)
            .iter()
            .map(|s| {
                let (s1, s2) = s.split_once(" | ").unwrap();
                (
                    s1.split(' ')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                    s2.split(' ')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                )
            })
            .collect::<Vec<(Vec<String>, Vec<String>)>>();
        let mut sum = 0;

        for (l1, l2) in lines {
            let mut hash_map: HashMap<String, i32> = HashMap::new();
            let ll1 = l1.clone();
            let eight = ll1.iter().find(|a| a.len() == 7).unwrap();
            let one = ll1.iter().find(|a| a.len() == 2).unwrap();
            let four = ll1.iter().find(|a| a.len() == 4).unwrap();
            let seven = ll1.iter().find(|a| a.len() == 3).unwrap();

            println!("{:?}", eight.chars().collect::<Vec<char>>());

            for d in l1 {
                let digit = match d.len() {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    5 => {
                        // acedgfb: 8
                        // cdfbe: 5
                        // gcdfa: 2
                        // fbcad: 3
                        // dab: 7
                        // cefabd: 9
                        // cdfgeb: 6
                        // eafb: 4
                        // cagedb: 0
                        // ab: 1

                        // cdfbe = 5
                        // gcdfa = 2
                        // fbcad = 3

                        // cdfgeb = 6
                        // cagedb = 0
                        // cefabd = 9

                        // let c1 = one.chars().collect::<Vec<char>>();
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

            // println!("{:?}", hash_map);

            let mut string = String::default();
            for ll2 in l2.into_iter() {
                for (k, v) in hash_map.iter() {
                    let mut a = k.as_bytes().to_vec();
                    a.sort();

                    let mut b = ll2.as_bytes().to_vec();
                    b.sort();
                    if a == b {
                        // println!("FOUND {} {} {}", k, ll2, v);
                        string = format!("{}{}", string, v);
                        break;
                    }
                }
            }

            sum += string.parse::<i32>().unwrap();

            // println!("{}", string);
        }

        Ok(Box::new(sum))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day8::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day8_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day8_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
