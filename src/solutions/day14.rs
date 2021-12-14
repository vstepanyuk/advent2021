use crate::solutions::{Result, Solution};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn solve(&self, input: Option<String>, steps: usize) -> usize {
        let lines = input.unwrap().lines().map(String::from).collect::<Vec<_>>();

        let polymer = lines.first().unwrap().to_string();
        let rules: HashMap<String, (String, Vec<String>)> =
            HashMap::from_iter(lines.iter().skip(2).map(|line| {
                let (pair, letter) = line.split_once(" -> ").unwrap();
                let (first, last) = pair.split_at(1);

                let first = format!("{}{}", first, letter);
                let last = format!("{}{}", letter, last);

                (pair.to_string(), (letter.to_string(), vec![first, last]))
            }));

        let mut polymer_pairs: HashMap<String, usize> = HashMap::new();
        let mut letters: HashMap<String, usize> = HashMap::new();

        polymer
            .chars()
            .map(|ch| {
                *letters.entry(ch.to_string()).or_insert(0) += 1;
                ch
            })
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|pair| {
                let pair = pair.iter().collect::<_>();
                *polymer_pairs.entry(pair).or_insert(0) += 1;
            });

        for _ in 0..steps {
            let mut tmp = HashMap::<String, usize>::new();
            polymer_pairs.iter().for_each(|(pair, count)| {
                let (letter, new_pairs) = rules.get(pair).unwrap();
                *letters.entry(letter.clone()).or_insert(0) += count;
                new_pairs.iter().for_each(|pair| {
                    *tmp.entry(pair.clone()).or_insert(0) += count;
                });
            });

            polymer_pairs = tmp;
        }

        letters.values().max().unwrap() - letters.values().min().unwrap()
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 10)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 40)))
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

        assert_eq!("1588", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day14_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("2188189693529", result.to_string())
    }
}
