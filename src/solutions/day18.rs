use std::fmt::{Debug, Display};

use itertools::Itertools;
use json::JsonValue;

use crate::day18::Item::{Close, Comma, Value};
use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug)]
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

trait Snailfish {
    fn from_string(input: &str) -> Vec<Item>;
    fn to_string(&self) -> String;
    fn add(&mut self, other: Vec<Item>);

    fn explode(&mut self) -> bool;
    fn split(&mut self) -> bool;
    fn reduce(&mut self);
}

impl Snailfish for Vec<Item> {
    fn from_string(input: &str) -> Vec<Item> {
        input
            .chars()
            .map(|ch| match ch {
                '[' => Item::Open,
                ']' => Item::Close,
                ',' => Item::Comma,
                x => Item::Value(x.to_digit(10).unwrap_or(0) as usize),
            })
            .collect()
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

    fn add(&mut self, other: Vec<Item>) {
        self.insert(0, Item::Open);
        self.push(Item::Comma);
        self.extend(other);
        self.push(Item::Close)
    }

    fn explode(&mut self) -> bool {
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
            return false;
        }

        let explode_position = explode_position.unwrap();

        let left = self[explode_position + 1].get_value();
        let right = self[explode_position + 3].get_value();

        self[explode_position] = Item::Value(0);

        for _ in 0..4 {
            self.remove(explode_position + 1);
        }

        let mut index = explode_position - 1;
        loop {
            if let Item::Value(value) = self[index] {
                self[index] = Item::Value(value + left);
                break;
            }

            if index == 0 {
                break;
            }
            index -= 1;
        }

        index = explode_position + 1;

        while index < self.len() {
            if let Item::Value(value) = self[index] {
                self[index] = Item::Value(value + right);
                break;
            }
            index += 1;
        }

        true
    }

    fn split(&mut self) -> bool {
        if let Some((index, item)) = self
            .iter()
            .find_position(|&item| matches!(item, Item::Value(v) if *v > 9))
        {
            let value = item.get_value();
            let a = value / 2;
            let b = value - a;

            self[index] = Item::Open;
            self.insert(index + 1, Value(a));
            self.insert(index + 2, Comma);
            self.insert(index + 3, Value(b));
            self.insert(index + 4, Close);

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        let mut same = false;
        while !same {
            same = true;
            while self.explode() {
                same = false;
            }

            if self.split() {
                same = false;
            }
        }
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
            current.add(Vec::from_string(line));
            current.reduce();
        }

        let value = json::parse(&current.to_string()).unwrap();

        Ok(Box::new(self.magnitude(value) as usize))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);

        let max = lines
            .iter()
            .permutations(2)
            .map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let mut a = Vec::from_string(a);

                a.add(Vec::from_string(b));
                a.reduce();

                let json = json::parse(&a.to_string()).unwrap();
                self.magnitude(json) as usize
            })
            .max()
            .unwrap();

        Ok(Box::new(max))
    }
}

#[cfg(test)]
mod tests {
    use crate::day18::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day18_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("4140", result.to_string());

        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("3488", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day18_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("3993", result.to_string());
    }
}
