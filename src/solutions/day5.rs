use std::cmp::max;
use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point(i32, i32);

impl Point {
    fn move_by(&mut self, offset: (i32, i32)) {
        self.0 += offset.0;
        self.1 += offset.1;
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or_default();
        return Ok(Point(x.parse()?, y.parse()?));
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    #[inline]
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    #[inline]
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn direction(&self) -> (i32, i32) {
        match (self.start.0, self.start.1, self.end.0, self.end.1) {
            (x1, y1, x2, y2) if x1 == x2 && y1 < y2 => (0, 1),
            (x1, y1, x2, y2) if x1 == x2 && y1 > y2 => (0, -1),
            (x1, y1, x2, y2) if y1 == y2 && x1 < x2 => (1, 0),
            (x1, y1, x2, y2) if y1 == y2 && x1 > x2 => (-1, 0),
            (x1, y1, x2, y2) if x1 < x2 && y1 < y2 => (1, 1),
            (x1, y1, x2, y2) if x1 > x2 && y1 < y2 => (-1, 1),
            (x1, y1, x2, y2) if x1 < x2 && y1 > y2 => (1, -1),
            _ => (-1, -1),
        }
    }
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (s, e) = s.split_once(" -> ").unwrap_or_default();
        return Ok(Segment {
            start: s.parse()?,
            end: e.parse()?,
        });
    }
}

trait Map: Debug {
    fn space_size(&self) -> (i32, i32);
}

impl Map for Vec<Segment> {
    fn space_size(&self) -> (i32, i32) {
        let (width, height) = self.iter().fold((0, 0), |(w, h), s| {
            (
                max(w, max(s.start.0, s.end.0)),
                max(h, max(s.start.1, s.end.1)),
            )
        });

        (width + 1, height + 1)
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let segments = parse_lines::<Segment>(input);
        let (width, height) = segments.space_size();

        let mut board = vec![0; (width * height) as usize];
        segments
            .iter()
            .filter(|s| s.is_horizontal() || s.is_vertical())
            .for_each(|s| {
                let direction = s.direction();
                let mut start = s.start.clone();

                while start != s.end {
                    board[(start.0 + start.1 * width) as usize] += 1;
                    start.move_by(direction);
                }

                board[(start.0 + start.1 * width) as usize] += 1;
            });

        let result = board.iter().filter(|&&v| v >= 2).count();
        Ok(Box::new(result))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let segments = parse_lines::<Segment>(input);
        let (width, height) = segments.space_size();

        let mut board = vec![0; (width * height) as usize];
        segments.iter().for_each(|s| {
            let direction = s.direction();
            let mut start = s.start.clone();

            while start != s.end {
                board[(start.0 + start.1 * width) as usize] += 1;
                start.move_by(direction);
            }

            board[(start.0 + start.1 * width) as usize] += 1;
        });

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
