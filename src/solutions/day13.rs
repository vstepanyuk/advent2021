use std::fmt::Display;
use std::ops::BitOr;

use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug)]
enum Flip {
    Horizontal(usize),
    Vertical(usize),
}

trait Flippable<T> {
    fn flip(&self, axis: &Flip) -> Matrix<T>;
}

impl<T> Flippable<T> for Matrix<T>
where
    T: Default + Copy + BitOr<Output = T>,
{
    fn flip(&self, axis: &Flip) -> Matrix<T> {
        let (width, height) = match axis {
            Flip::Vertical(value) => (self.width, *value),
            Flip::Horizontal(value) => (*value, self.height),
        };

        let mut new_matrix: Matrix<T> = Matrix::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let (xx, yy) = match axis {
                    Flip::Horizontal(_) => (self.width - x - 1, y),
                    Flip::Vertical(_) => (x, self.height - y - 1),
                };

                let value1 = *self.get(x, y).unwrap();
                let value2 = *self.get(xx, yy).unwrap();

                new_matrix.set(x, y, value1 | value2);
            }
        }

        new_matrix
    }
}

impl DaySolution {
    fn parse(&self, input: Option<String>) -> (Matrix<u8>, Vec<Flip>) {
        let input = input.unwrap();

        let mut width = 0;
        let mut height = 0;

        let points = input
            .lines()
            .take_while(|&line| !line.is_empty())
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

                if x > width {
                    width = x;
                }

                if y > height {
                    height = y;
                }

                (x, y)
            })
            .collect::<Vec<_>>();

        let flips = input
            .lines()
            .skip_while(|&line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let (_, value) = line.split_once('=').unwrap();
                let value = value.parse().unwrap();
                match line {
                    _ if line.starts_with("fold along y=") => Flip::Vertical(value),
                    _ => Flip::Horizontal(value),
                }
            })
            .collect::<Vec<_>>();

        let mut matrix = Matrix::new(width + 1, height + 1);
        points.iter().for_each(|(x, y)| {
            matrix.set(*x, *y, 1);
        });

        (matrix, flips)
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (mut matrix, flips) = self.parse(input);

        flips.iter().take(1).for_each(|flip| {
            matrix = matrix.flip(flip);
        });

        let count = matrix.iter().filter(|(&v, _)| v == 1).count();
        Ok(Box::new(count))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (mut matrix, flips) = self.parse(input);

        flips.iter().for_each(|flip| {
            matrix = matrix.flip(flip);
        });

        let result = matrix.render_to_string(|value| match value {
            Some(&1) => "#".to_string(),
            _ => ".".to_string(),
        });

        Ok(Box::new(format!("\n{}", result)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day13_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("17", result.to_string());

        let input = include_str!("../../inputs/day13.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("664", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day13_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!(
            "
#####
#...#
#...#
#...#
#####
.....
.....",
            result.to_string()
        );

        let input = include_str!("../../inputs/day13.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!(
            "
####.####...##.#..#.####.#....###..#....
#....#.......#.#.#.....#.#....#..#.#....
###..###.....#.##.....#..#....###..#....
#....#.......#.#.#...#...#....#..#.#....
#....#....#..#.#.#..#....#....#..#.#....
####.#.....##..#..#.####.####.###..####.",
            result.to_string()
        );
    }
}
