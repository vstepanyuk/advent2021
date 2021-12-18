use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use itertools::{iproduct, Itertools};
use json::JsonValue;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::many1;
use nom::{combinator::map_res, IResult};
use std::fmt::{Debug, Display};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Item {
    Open,
    Close,
    Comma,
    Value(usize),
}

impl Item {
    fn get_value(&self) -> usize {
        match self {
            Self::Value(value) => *value,
            _ => unreachable!(),
        }
    }
}

fn parse_usize(input: &str) -> std::result::Result<usize, std::num::ParseIntError> {
    input.parse::<usize>()
}

impl Item {
    fn comma(input: &str) -> IResult<&str, Item> {
        map(tag(","), |_| Self::Comma)(input)
    }

    fn open(input: &str) -> IResult<&str, Item> {
        map(tag("["), |_| Self::Open)(input)
    }

    fn close(input: &str) -> IResult<&str, Item> {
        map(tag("]"), |_| Self::Close)(input)
    }

    fn value(input: &str) -> IResult<&str, Item> {
        map(map_res(digit1, parse_usize), Self::Value)(input)
    }

    fn any(input: &str) -> IResult<&str, Item> {
        alt((Self::open, Self::close, Self::comma, Self::value))(input)
    }
}

trait Snailfish {
    fn from_string(input: &str) -> Vec<Item>;
    fn to_string(&self) -> String;
    fn explode(&self) -> Vec<Item>;
    fn split(&self) -> Vec<Item>;
    fn add(&self, other: Vec<Item>) -> Vec<Item>;
    fn reduce(&self) -> Vec<Item>;
}

impl Snailfish for Vec<Item> {
    fn from_string(input: &str) -> Vec<Item> {
        many1(Item::any)(input).unwrap_or_default().1
    }

    fn to_string(&self) -> String {
        self.iter()
            .map(|i| match i {
                Item::Open => "[".to_string(),
                Item::Close => "]".to_string(),
                Item::Comma => ",".to_string(),
                Item::Value(value) => format!("{}", value),
            })
            .join("")
    }

    fn explode(&self) -> Vec<Item> {
        let mut open_count = 0;
        let mut explode_position = None;

        for (index, item) in self.iter().enumerate() {
            open_count += match item {
                Item::Open => 1,
                Item::Close => -1,
                _ => 0,
            };

            if open_count == 5 {
                explode_position = Some(index);
                break;
            }
        }

        if explode_position.is_none() {
            return self.to_vec();
        }

        let explode_position = explode_position.unwrap();

        let left = self[explode_position + 1].get_value();
        let right = self[explode_position + 3].get_value();

        let mut result = vec![Item::Value(0)];
        let mut added = false;

        let mut index = explode_position - 1;
        loop {
            match self[index] {
                Item::Value(value) if !added => {
                    result.insert(0, Item::Value(value + left));
                    added = true;
                }
                _ => result.insert(0, self[index]),
            };

            if index == 0 {
                break;
            }
            index -= 1;
        }

        index = explode_position + 5;
        added = false;

        while index < self.len() {
            match self[index] {
                Item::Value(value) if !added => {
                    result.push(Item::Value(value + right));
                    added = true;
                }
                _ => result.push(self[index]),
            };
            index += 1;
        }

        result
    }

    fn split(&self) -> Vec<Item> {
        let mut has_split = false;

        self.iter()
            .flat_map(|item| match item {
                Item::Value(value) if *value > 9 && !has_split => {
                    has_split = true;

                    let a = value / 2;
                    let b = value - a;

                    vec![
                        Item::Open,
                        Item::Value(a),
                        Item::Comma,
                        Item::Value(b),
                        Item::Close,
                    ]
                }
                _ => vec![*item],
            })
            .collect()
    }

    fn add(&self, other: Vec<Item>) -> Vec<Item> {
        let mut result = vec![Item::Open];
        result.extend(self);
        result.push(Item::Comma);
        result.extend(other);
        result.push(Item::Close);

        result
    }

    fn reduce(&self) -> Vec<Item> {
        let mut last = self.clone();
        loop {
            let mut last_explode = last.clone();
            loop {
                let tmp = last_explode.explode();
                if tmp == last_explode {
                    last = tmp;
                    break;
                }

                last_explode = tmp;
            }

            last_explode = last_explode.split();
            if last_explode == last {
                break;
            }

            last = last_explode
        }

        last
    }
}

impl DaySolution {
    fn magnitude(&self, value: JsonValue) -> f64 {
        match value {
            JsonValue::Array(arr) => {
                let a = arr[0].clone();
                let b = arr[1].clone();

                match (a, b) {
                    (JsonValue::Number(a), JsonValue::Number(b)) => {
                        let a: f64 = a.into();
                        let b: f64 = b.into();
                        a * 3.0 + b * 2.0
                    }
                    (a, JsonValue::Number(b)) => {
                        let b: f64 = b.into();
                        self.magnitude(a) + b * 2.0
                    }
                    (JsonValue::Number(a), b) => {
                        let a: f64 = a.into();
                        a * 3.0 + self.magnitude(b)
                    }
                    (a, b) => self.magnitude(a) * 3.0 + self.magnitude(b) * 2.0,
                }
            }
            _ => 0.0,
        }
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);
        let mut current = Vec::<Item>::from_string(&lines[0]);

        for line in lines.iter().skip(1) {
            current = current.add(Vec::from_string(line));
            current = current.reduce();
        }

        let value = json::parse(&current.to_string()).unwrap();

        Ok(Box::new(self.magnitude(value) as usize))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);

        let max = iproduct!(lines.iter(), lines.iter().skip(1))
            .map(|(a, b)| {
                let a = Vec::from_string(a);
                let b = Vec::from_string(b);

                let json = json::parse(&a.add(b).reduce().to_string()).unwrap();
                self.magnitude(json) as usize
            })
            .max()
            .unwrap();

        Ok(Box::new(max))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day18::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day18_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day18_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
