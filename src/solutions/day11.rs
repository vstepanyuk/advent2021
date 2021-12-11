use crate::helpers::*;
use crate::solutions::{Result, Solution};
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution {}

impl DaySolution {
    fn step(&self, matrix: &mut Matrix<i32>) -> usize {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();

        for y in 0..matrix.height {
            for x in 0..matrix.width {
                let v = *matrix.get(x, y).unwrap();
                matrix.set(x, y, v + 1);

                if (v + 1) > 9 {
                    q.push_back((x as i32, y as i32));
                }
            }
        }

        let offsets = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        let mut visited = HashSet::new();
        while let Some(pos) = q.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);
            let (x, y) = pos;
            for o in offsets {
                let xx = x + o.0;
                let yy = y + o.1;
                if xx < 0
                    || (xx as usize) >= matrix.width
                    || yy < 0
                    || (yy as usize) >= matrix.height
                {
                    continue;
                }

                let v = *matrix.get(xx, yy).unwrap();

                matrix.set(xx, yy, v + 1);
                if (v + 1) > 9 {
                    q.push_back((xx, yy))
                }
            }
        }

        for x in 0..matrix.width {
            for y in 0..matrix.height {
                let v = matrix.get(x, y).unwrap();

                if *v > 9 {
                    matrix.set(x, y, 0);
                }
            }
        }

        visited.len()
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();

        let mut c = 0;
        for step in 0..100 {
            c += self.step(&mut matrix);
            println!("{:?}", matrix);
        }

        Ok(Box::new(c))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut matrix = Matrix::<i32>::from(&input.unwrap()).unwrap();

        let mut c = 0;
        while self.step(&mut matrix) != matrix.width * matrix.height {
            c += 1;
        }
        Ok(Box::new(c + 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day11_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("1656", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day11_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("195", result.to_string())
    }
}
