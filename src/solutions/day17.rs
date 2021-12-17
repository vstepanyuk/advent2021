use crate::solutions::{Result, Solution};
use std::fmt::Display;
use std::ops::RangeInclusive;

#[derive(Default)]
pub struct DaySolution;

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

        let mut yy = i32::MIN;
        loop {
            x += vx;
            y += vy;

            if vx > 0 {
                vx -= 1;
            }

            vy -= 1;

            if y > yy {
                yy = y;
            }

            if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                return Some(yy);
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
        let input = input.unwrap();
        let (_, input) = input.split_once(": ").unwrap();
        let (x_range, y_range) = input.split_once(", ").unwrap();

        let x_range = x_range.replace("x=", "");
        let y_range = y_range.replace("y=", "");

        let (x_min, x_max) = x_range.split_once("..").unwrap();
        let (y_min, y_max) = y_range.split_once("..").unwrap();

        let (x_min, x_max) = (x_min.parse::<i32>().unwrap(), x_max.parse::<i32>().unwrap());
        let (y_min, y_max) = (y_min.parse::<i32>().unwrap(), y_max.parse::<i32>().unwrap());

        let mut mm = i32::MIN;
        for i in 0..=x_max {
            for j in y_min..x_max {
                if let Some(y) = self.hit((i, j), x_min, x_max, y_min, y_max) {
                    if y > mm {
                        mm = y;
                    }
                }
            }
        }

        Ok(Box::new(mm))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let input = input.unwrap();
        let (_, input) = input.split_once(": ").unwrap();
        let (x_range, y_range) = input.split_once(", ").unwrap();

        let x_range = x_range.replace("x=", "");
        let y_range = y_range.replace("y=", "");

        let (x_min, x_max) = x_range.split_once("..").unwrap();
        let (y_min, y_max) = y_range.split_once("..").unwrap();

        let (x_min, x_max) = (x_min.parse::<i32>().unwrap(), x_max.parse::<i32>().unwrap());
        let (y_min, y_max) = (y_min.parse::<i32>().unwrap(), y_max.parse::<i32>().unwrap());

        let mut count = 0;
        for i in 0..=x_max {
            for j in y_min..x_max {
                if let Some(y) = self.hit((i, j), x_min, x_max, y_min, y_max) {
                    count += 1;
                }
            }
        }

        Ok(Box::new(count))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day17::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day17_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day17_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
