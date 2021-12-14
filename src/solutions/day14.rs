use crate::solutions::{Result, Solution};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Default)]
pub struct DaySolution;

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let lines = input.lines().map(String::from).collect::<Vec<_>>();
        let mut polymer = lines.first().unwrap().to_string();

        let rules = lines
            .iter()
            .skip(2)
            .map(|line| {
                let (a, b) = line.split_once(" -> ").unwrap();
                return (a.to_string(), b.to_string());
            })
            .collect::<Vec<(String, String)>>();

        for _ in 0..10 {
            let mut pp = String::default();

            polymer
                .chars()
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|a| {
                    let b = a.to_vec().iter().collect::<String>();
                    let rule = rules
                        .iter()
                        .find(|(r, _)| *r == b)
                        .map(|(_, r)| r.to_owned())
                        .unwrap();

                    pp = format!("{}{}{}", pp, a[0], rule);
                });
            pp = format!("{}{}", pp, polymer.chars().last().unwrap());

            polymer = pp;
        }

        let mut hashmap: HashMap<char, usize> = HashMap::new();
        for ch in polymer.chars() {
            *hashmap.entry(ch).or_insert(0) += 1;
        }

        let a = hashmap.values().min().unwrap();
        let b = hashmap.values().max().unwrap();

        println!("a={}, b={}", a, b);

        Ok(Box::new(b - a))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let lines = input.lines().map(String::from).collect::<Vec<_>>();
        let mut polymer = lines.first().unwrap().to_string();

        let rules = lines
            .iter()
            .skip(2)
            .map(|line| {
                let (a, b) = line.split_once(" -> ").unwrap();
                let chars = a.chars().collect::<Vec<_>>();

                let c1 = format!("{}{}", chars[0], b);
                let c2 = format!("{}{}", b, chars[1]);

                return (a.to_string(), b.to_string(), c1, c2);
            })
            .collect::<Vec<(String, String, String, String)>>();

        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut letters = HashMap::<char, usize>::new();

        polymer
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|pair| {
                let pair = pair.to_vec().iter().collect::<String>();
                *counts.entry(pair).or_insert(0) += 1;
            });

        polymer.chars().for_each(|ch| {
            *letters.entry(ch).or_insert(0) += 1;
        });

        let mut rules2: HashMap<String, (String, String)> = HashMap::new();

        for rule in rules {
            rules2.entry(rule.0.clone()).or_insert((rule.2, rule.3));
        }

        for _ in 0..40 {
            // println!("{:?}", counts);
            // println!("{:?}", letters);

            let mut counts2: HashMap<String, usize> = HashMap::new();
            counts.iter().for_each(|(pair, c)| {
                if *c == 0 {
                    return;
                }

                let cc = pair.chars().collect::<Vec<_>>();

                for ccc in cc {
                    *letters.entry(ccc).or_insert(0) -= c;
                }

                let (r1, r2) = rules2.get(pair).unwrap();
                let c1 = r1.chars().collect::<Vec<_>>()[0];
                let c2 = r2.chars().collect::<Vec<_>>()[1];
                let c3 = r2.chars().collect::<Vec<_>>()[0];

                *letters.entry(c1).or_insert(0) += c;
                *letters.entry(c2).or_insert(0) += c;
                *letters.entry(c3).or_insert(0) += c;

                *counts2.entry(r1.clone()).or_insert(0) += *c;
                *counts2.entry(r2.clone()).or_insert(0) += *c;
            });

            counts = counts2;
        }
        println!("{:?}", letters);

        let a = letters.values().min().unwrap();
        let b = letters.values().max().unwrap();

        println!("{:?}, {}", letters, b - a);

        Ok(Box::new(0))
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day14_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day14_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("", result.to_string())
    }
}
