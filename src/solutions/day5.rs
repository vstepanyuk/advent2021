use crate::helpers::parse_lines;
use std::cmp::{max, min};
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::prelude::rust_2015::Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or_default();

        return Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        });
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::prelude::rust_2015::Result<Self, Self::Err> {
        let (s, e) = s.split_once(" -> ").unwrap_or_default();
        return Ok(Line {
            start: s.parse()?,
            end: e.parse()?,
        });
    }
}

trait Map {
    fn print(&self, width: usize);
}

impl Map for Vec<i32> {
    fn print(&self, width: usize) {
        let height = self.len() / width;

        for y in 0..height {
            for x in 0..width {
                print!("{}\t", self[x + y * width]);
            }
            println!();
        }
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<Line>(input);

        let width = lines.iter().map(|l| max(l.start.x, l.end.x)).max().unwrap() + 1;
        let height = lines.iter().map(|l| max(l.start.y, l.end.y)).max().unwrap() + 1;

        let mut board: Vec<i32> = vec![0; (width * height) as usize];
        for line in lines {
            let (min_v, max_v) = if line.start.y == line.end.y {
                (min(line.start.x, line.end.x), max(line.start.x, line.end.x))
            } else {
                (min(line.start.y, line.end.y), max(line.start.y, line.end.y))
            };

            for i in min_v..=max_v {
                if !(line.start.y == line.end.y || line.start.x == line.end.x) {
                    continue;
                }

                if line.start.y == line.end.y {
                    board[(i + line.start.y * width) as usize] += 1;
                } else {
                    board[(line.start.x + i * width) as usize] += 1;
                }
            }
        }

        let result = board.iter().filter(|&&v| v >= 2).count();

        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<Line>(input);

        let width = lines.iter().map(|l| max(l.start.x, l.end.x)).max().unwrap() + 1;
        let height = lines.iter().map(|l| max(l.start.y, l.end.y)).max().unwrap() + 1;

        let mut board: Vec<i32> = vec![0; (width * height) as usize];
        for line in lines {
            let (min_x, max_x) = (min(line.start.x, line.end.x), max(line.start.x, line.end.x));
            let (min_y, max_y) = (min(line.start.y, line.end.y), max(line.start.y, line.end.y));

            if line.start.y == line.end.y {
                for i in min_x..=max_x {
                    board[(i + line.start.y * width) as usize] += 1;
                }
            } else if line.start.x == line.end.x {
                for i in min_y..=max_y {
                    board[(line.start.x + i * width) as usize] += 1;
                }
            } else {
                if (line.start.y - line.end.y > 0 && line.start.x - line.end.x > 0)
                    || (line.start.y - line.end.y < 0 && line.start.x - line.end.x < 0)
                {
                    for i in 0..=(max_x - min_x) {
                        board[(min_x + i + (min_y + i) * width) as usize] += 1;
                    }
                } else {
                    for i in 0..=(max_x - min_x) {
                        board[(max_x - i + (min_y + i) * width) as usize] += 1;
                    }
                }
            }
        }

        let result = board.iter().filter(|&&v| v >= 2).count();

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day5_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("5", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day5_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("12", result.to_string())
    }
}
