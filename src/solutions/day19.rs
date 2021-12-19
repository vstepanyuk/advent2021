use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::str::FromStr;

use itertools::{iproduct, Itertools};

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Position(i32, i32, i32);

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Position(x, y, z)
    }

    fn rotate(&self, index: usize) -> Option<Position> {
        match index {
            0 => Some(Position::new(self.0, self.1, self.2)),
            1 => Some(Position::new(self.0, self.1, -self.2)),
            2 => Some(Position::new(self.0, -self.1, self.2)),
            3 => Some(Position::new(self.0, -self.1, -self.2)),
            4 => Some(Position::new(-self.0, self.1, self.2)),
            5 => Some(Position::new(-self.0, self.1, -self.2)),
            6 => Some(Position::new(-self.0, -self.1, self.2)),
            7 => Some(Position::new(-self.0, -self.1, -self.2)),
            8 => Some(Position::new(self.2, self.0, self.1)),
            9 => Some(Position::new(self.2, self.0, -self.1)),
            10 => Some(Position::new(self.2, -self.0, self.1)),
            11 => Some(Position::new(self.2, -self.0, -self.1)),
            12 => Some(Position::new(-self.2, self.0, self.1)),
            13 => Some(Position::new(-self.2, self.0, -self.1)),
            14 => Some(Position::new(-self.2, -self.0, self.1)),
            15 => Some(Position::new(-self.2, -self.0, -self.1)),
            16 => Some(Position::new(self.1, self.2, self.0)),
            17 => Some(Position::new(self.1, self.2, -self.0)),
            18 => Some(Position::new(self.1, -self.2, self.0)),
            19 => Some(Position::new(self.1, -self.2, -self.0)),
            20 => Some(Position::new(-self.1, self.2, self.0)),
            21 => Some(Position::new(-self.1, self.2, -self.0)),
            22 => Some(Position::new(-self.1, -self.2, self.0)),
            23 => Some(Position::new(-self.1, -self.2, -self.0)),
            24 => Some(Position::new(self.0, self.2, self.1)),
            25 => Some(Position::new(self.0, self.2, -self.1)),
            26 => Some(Position::new(self.0, -self.2, self.1)),
            27 => Some(Position::new(self.0, -self.2, -self.1)),
            28 => Some(Position::new(-self.0, self.2, self.1)),
            29 => Some(Position::new(-self.0, self.2, -self.1)),
            30 => Some(Position::new(-self.0, -self.2, self.1)),
            31 => Some(Position::new(-self.0, -self.2, -self.1)),
            32 => Some(Position::new(self.1, self.0, self.2)),
            33 => Some(Position::new(self.1, self.0, -self.2)),
            34 => Some(Position::new(self.1, -self.0, self.2)),
            35 => Some(Position::new(self.1, -self.0, -self.2)),
            36 => Some(Position::new(-self.1, self.0, self.2)),
            37 => Some(Position::new(-self.1, self.0, -self.2)),
            38 => Some(Position::new(-self.1, -self.0, self.2)),
            39 => Some(Position::new(-self.1, -self.0, -self.2)),
            40 => Some(Position::new(self.2, self.1, self.0)),
            41 => Some(Position::new(self.2, self.1, -self.0)),
            42 => Some(Position::new(self.2, -self.1, self.0)),
            43 => Some(Position::new(self.2, -self.1, -self.0)),
            44 => Some(Position::new(-self.2, self.1, self.0)),
            45 => Some(Position::new(-self.2, self.1, -self.0)),
            46 => Some(Position::new(-self.2, -self.1, self.0)),
            47 => Some(Position::new(-self.2, -self.1, -self.0)),
            _ => None,
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[derive(Default, Debug, Clone)]
struct Scanner {
    beacons: Vec<Position>,
}

impl Scanner {
    fn rotations(&self) -> impl Iterator<Item = Scanner> + '_ {
        ScannerRotationsIterator::new(self)
    }
}

struct ScannerRotationsIterator<'a> {
    scanner: &'a Scanner,
    index: usize,
}

impl<'a> ScannerRotationsIterator<'a> {
    fn new(scanner: &'a Scanner) -> Self {
        Self { scanner, index: 0 }
    }
}

impl Iterator for ScannerRotationsIterator<'_> {
    type Item = Scanner;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 48 {
            return None;
        }

        let beacons = self
            .scanner
            .beacons
            .iter()
            .map(|pos| pos.rotate(self.index).unwrap())
            .collect();

        self.index += 1;

        Some(Scanner { beacons })
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let result = s
            .split(',')
            .map(|coord| coord.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Ok(Position::new(result[0], result[1], result[2]))
    }
}

impl DaySolution {
    fn parse(&self, input: String) -> Vec<Scanner> {
        let mut result = vec![];

        let mut scanner = Scanner::default();
        for line in input.lines().skip(1) {
            if line.is_empty() {
                result.push(scanner.clone());
                continue;
            }

            if line.starts_with("--- scanner ") {
                scanner = Scanner::default();
                continue;
            }

            let position = line.parse::<Position>().unwrap();

            scanner.beacons.push(position);
        }

        result
    }

    fn find(&self, scanner: &Scanner, beacons: &HashSet<Position>) -> Option<Position> {
        iproduct!(beacons.iter(), &scanner.beacons)
            .map(|(a, b)| *a - *b)
            .find(|offset| {
                scanner
                    .beacons
                    .iter()
                    .filter(|&pos| beacons.contains(&(*pos + *offset)))
                    .take(12)
                    .count()
                    == 12
            })
    }

    fn solve(&self, input: Option<String>) -> (HashSet<Position>, Vec<Position>) {
        let scanners = self.parse(input.unwrap());
        let first_scanner = scanners.first().unwrap();

        let mut beacons = HashSet::<Position>::from_iter(first_scanner.beacons.to_vec());
        let mut skip = HashSet::from([0]);
        let mut offsets = vec![];

        loop {
            let found = scanners
                .iter()
                .enumerate()
                .filter(|(index, _)| !skip.contains(index))
                .find_map(|(index, scanner)| {
                    let found = scanner.rotations().find_map(|rotation| {
                        self.find(&rotation, &beacons)
                            .map(|offset| (offset, rotation))
                    });

                    found.map(|(offset, scanner)| (offset, scanner, index))
                });

            if let Some((offset, scanner, index)) = found {
                beacons.extend(scanner.beacons.iter().map(|pos| *pos + offset));
                skip.insert(index);
                offsets.push(offset);
            } else {
                break;
            }
        }

        (beacons, offsets)
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input).0.len()))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(
            self.solve(input)
                .1
                .iter()
                .tuple_combinations()
                .map(|(&a, &b)| (a - b).manhattan_distance())
                .max()
                .unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::day19::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day19_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("79", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day19_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("3621", result.to_string())
    }
}
