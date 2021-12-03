use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn count_bits<'a>(&self, input: &[String], at: usize) -> (i32, i32) {
        input.iter().fold((0, 0), |bits, line| {
            let bit_set = (line.as_bytes()[at] == b'1') as i32;

            (bits.0 + 1 - bit_set, bits.1 + bit_set)
        })
    }

    fn retain<T, F>(&self, arr: &mut Vec<T>, mut func: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut count = arr.len();
        arr.retain(|elem| {
            let is_deleted = func(elem);
            count -= is_deleted as usize;

            if count == 0 {
                return true;
            }

            is_deleted
        })
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<()> {
        let lines: Vec<String> = input
            .unwrap_or_default()
            .lines()
            .map(String::from)
            .collect();

        let len = lines.first().map(String::len).unwrap_or(0);
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..len {
            let bits = self.count_bits(&lines, i);
            let bit = (bits.1 > bits.0) as i32;

            gamma |= bit << (len - i - 1);
            epsilon |= (1 - bit) << (len - i - 1);
        }

        println!("{}", gamma * epsilon);
        Ok(())
    }

    fn part_2(&mut self, input: Option<String>) -> Result<()> {
        let lines: Vec<String> = input
            .unwrap_or_default()
            .lines()
            .map(String::from)
            .collect();

        let len = lines.first().map(String::len).unwrap_or(0);

        let mut oxygen = lines.clone();
        let mut co2 = lines.clone();

        for i in 0..len {
            if oxygen.len() > 1 {
                let bits = self.count_bits(&oxygen, i);

                self.retain(&mut oxygen, |line| match line.as_bytes()[i] {
                    b'0' if bits.0 < bits.1 || bits.0 == bits.1 => false,
                    b'1' if bits.0 > bits.1 => false,
                    _ => true,
                });
            }

            if co2.len() > 1 {
                let bits = self.count_bits(&co2, i);
                self.retain(&mut co2, |line| match line.as_bytes()[i] {
                    b'0' if bits.0 > bits.1 => false,
                    b'1' if bits.0 < bits.1 || bits.0 == bits.1 => false,
                    _ => true,
                });
            }
        }

        let oxygen = isize::from_str_radix(oxygen.first().unwrap(), 2).unwrap();
        let co2 = isize::from_str_radix(co2.first().unwrap(), 2).unwrap();

        println!("{}", oxygen * co2);

        Ok(())
    }
}
