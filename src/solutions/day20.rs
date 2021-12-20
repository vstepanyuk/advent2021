use itertools::iproduct;
use std::fmt::Display;

use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};

type Image = Matrix<u8>;

#[derive(Default)]
pub struct DaySolution {}

impl DaySolution {
    fn solve(&self, input: Option<String>, steps: usize) -> usize {
        let input = input.unwrap();
        let (enhancement, image_data) = input.split_once("\n\n").unwrap();
        let enhancement = enhancement.chars().collect::<Vec<_>>();
        let image = Image::from(&image_data.replace('#', "1").replace('.', "0")).unwrap();

        let mut new_image = Image::new(image.width + steps * 2, image.height + steps * 2);
        image.iter().for_each(|(value, (x, y))| {
            new_image.set(x + steps, y + steps, *value);
        });

        for step in 0..steps {
            new_image = Image::from_iter(
                new_image.iter().map(|(_, (x, y))| {
                    let index = iproduct!(-1..=1, -1..=1).fold(0usize, |result, (dy, dx)| {
                        (result << 1)
                            + match new_image.get(x as i32 + dx, y as i32 + dy) {
                                Some(x) => *x as usize,
                                None if enhancement[0] == '.' => 0,
                                None => step % 2,
                            }
                    });

                    (enhancement[index] == '#').then(|| &1).unwrap_or(&0)
                }),
                new_image.width,
            );
        }

        new_image.iter().filter(|(&v, _)| v == 1).count()
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 2)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 50)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day20_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("35", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day20_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("3351", result.to_string());
    }
}
