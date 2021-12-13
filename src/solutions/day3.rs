use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

trait BitString {
    fn bit_is_set(&self, at: usize) -> bool;
}

impl BitString for String {
    fn bit_is_set(&self, at: usize) -> bool {
        self.as_bytes()[at] == b'1'
    }
}

impl DaySolution {
    fn count_bits(&self, input: &[String], at: usize) -> (i32, i32) {
        input.iter().fold((0, 0), |bits, line| {
            let bit = line.bit_is_set(at) as i32;
            (bits.0 + 1 - bit, bits.1 + bit)
        })
    }

    fn retain_at_least_one<T, F>(&self, arr: &mut Vec<T>, mut func: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut count = arr.len();
        if count == 1 {
            return;
        }

        arr.retain(|elem| {
            let is_deleted = func(elem);
            count -= !is_deleted as usize;
            count == 0 || is_deleted
        })
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines(input);
        let len = lines.first().map(String::len).unwrap_or(0);
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..len {
            let bits = self.count_bits(&lines, i);
            let bit = (bits.1 > bits.0) as i32;

            gamma |= bit << (len - i - 1);
            epsilon |= (1 - bit) << (len - i - 1);
        }

        Ok(Box::new(gamma * epsilon))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines(input);
        let len = lines.first().map(String::len).unwrap_or(0);
        let mut oxygen = lines.clone();
        let mut co2 = lines;

        for i in 0..len {
            let bits = self.count_bits(&oxygen, i);
            self.retain_at_least_one(&mut oxygen, |line| {
                (!line.bit_is_set(i) || bits.0 <= bits.1) && (line.bit_is_set(i) || bits.0 > bits.1)
            });

            let bits = self.count_bits(&co2, i);
            self.retain_at_least_one(&mut co2, |line| {
                (line.bit_is_set(i) || bits.0 <= bits.1) && (!line.bit_is_set(i) || bits.0 > bits.1)
            });
        }

        let oxygen = oxygen
            .first()
            .map(|s| isize::from_str_radix(s, 2).unwrap_or(0))
            .unwrap_or(0);

        let co2 = co2
            .first()
            .map(|s| isize::from_str_radix(s, 2).unwrap_or(0))
            .unwrap_or(0);

        Ok(Box::new(oxygen * co2))
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day3_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("198", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day3_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("230", result.to_string())
    }
}
