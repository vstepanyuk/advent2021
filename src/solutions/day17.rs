use crate::solutions::{Result, Solution};
use itertools::iproduct;
use regex::Regex;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn parse(&self, input: Option<String>) -> (i32, i32, i32, i32) {
        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let input = input.unwrap();
        let caps = re.captures(&input).unwrap();

        (
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        )
    }
}

impl DaySolution {
    fn hit(
        &self,
        velocity: (i32, i32),
        x_min: i32,
        x_max: i32,
        y_min: i32,
        y_max: i32,
    ) -> Option<i32> {
        let (mut vx, mut vy) = velocity;
        let (mut x, mut y) = (0, 0);

        let mut max_y = 0;

        loop {
            x += vx;
            y += vy;

            vx -= vx.signum();
            vy -= 1;

            if y > max_y {
                max_y = y;
            }

            if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                return Some(max_y);
            }

            if x > x_max {
                break;
            }

            if y < y_min {
                break;
            }
        }

        None
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (x_min, x_max, y_min, y_max) = self.parse(input);

        let max_height = iproduct!(0..=x_max, y_min..x_max)
            .filter_map(|(dx, dy)| self.hit((dx, dy), x_min, x_max, y_min, y_max))
            .max()
            .unwrap();

        Ok(Box::new(max_height))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (x_min, x_max, y_min, y_max) = self.parse(input);

        let count = iproduct!(0..=x_max, y_min..x_max)
            .filter_map(|(dx, dy)| self.hit((dx, dy), x_min, x_max, y_min, y_max))
            .count();

        Ok(Box::new(count))
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day17_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("45", result.to_string());

        let input = include_str!("../../inputs/day17.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("5460", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day17_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("112", result.to_string());

        let input = include_str!("../../inputs/day17.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("3618", result.to_string());
    }
}
