use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn bits<'a>(&self, input: &[String], at: usize) -> (i32, i32) {
        input.iter().fold((0, 0), |bits, line| {
            let bytes = line.as_bytes();
            let bit0 = (bytes[at] == b'0') as i32;
            let bit1 = (bytes[at] == b'1') as i32;

            (bits.0 + bit0, bits.1 + bit1)
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

        let len = lines.first().map(|s| s.len()).unwrap_or_default();
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..len {
            let bits = self.bits(&lines, i);
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

        let len = lines.first().map(|s| s.len()).unwrap_or_default();

        let mut oxygen = lines.clone();
        let mut co2 = lines.clone();

        for i in 0..len {
            let bits = self.bits(&oxygen, i);

            if oxygen.len() > 1 {
                let remove = oxygen
                    .iter()
                    .enumerate()
                    .filter_map(|(index, line)| {
                        let chars = line.as_bytes();
                        match chars[i] {
                            b'0' if bits.0 < bits.1 || bits.0 == bits.1 => Some(index),
                            b'1' if bits.0 > bits.1 => Some(index),
                            _ => None,
                        }
                    })
                    .collect::<Vec<usize>>();

                remove.iter().rev().for_each(|i| {
                    oxygen.remove(*i);
                });
            }

            let bits = self.bits(&co2, i);
            if co2.len() > 1 {
                let remove = co2
                    .iter()
                    .enumerate()
                    .filter_map(|(index, line)| {
                        let chars = line.as_bytes();
                        match chars[i] {
                            b'0' if bits.0 > bits.1 => Some(index),
                            b'1' if bits.0 < bits.1 || bits.0 == bits.1 => Some(index),
                            _ => None,
                        }
                    })
                    .collect::<Vec<usize>>();

                remove.iter().rev().for_each(|i| {
                    co2.remove(*i);
                });
            }
        }

        let oxygen = isize::from_str_radix(oxygen.first().unwrap(), 2).unwrap();
        let co2 = isize::from_str_radix(co2.first().unwrap(), 2).unwrap();

        println!("{}", oxygen * co2);

        Ok(())
    }
}
