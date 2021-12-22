use crate::solutions::{Result, Solution};
use euclid::{Box3D, Point3D};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

type Cuboid = Box3D<isize, ()>;
type Point = Point3D<isize, ()>;

#[derive(Debug)]
struct Rule {
    cuboid: Box3D<isize, ()>,
    status: bool,
}

impl DaySolution {
    fn parse(&self, input: Option<String>) -> Vec<Rule> {
        let input = input.unwrap();
        let mut rules = vec![];

        for line in input.lines() {
            let (status, rest) = line.split_once(' ').unwrap();
            let status = status == "on";

            let result = rest
                .replace("x=", "")
                .replace("y=", "")
                .replace("z=", "")
                .split(',')
                .map(|str| {
                    let (mn, mx) = str.split_once("..").unwrap();
                    let mn = mn.parse::<isize>().unwrap();
                    let mx = mx.parse::<isize>().unwrap();

                    return (mn, mx);
                })
                .collect::<Vec<_>>();

            let cuboid = Cuboid::new(
                Point3D::new(result[0].0, result[1].0, result[2].0),
                Point3D::new(result[0].1, result[1].1, result[2].1),
            );

            rules.push(Rule { cuboid, status })
        }

        rules
    }

    fn generate(&self, cuboid: &Cuboid) -> Vec<Cuboid> {
        let mut result = vec![];

        for x in cuboid.min.x..=cuboid.max.x {
            for y in cuboid.min.y..=cuboid.max.y {
                for z in cuboid.min.z..=cuboid.max.z {
                    result.push(Cuboid::new(
                        Point3D::new(x, y, z),
                        Point3D::new(x + 1, y + 1, z + 1),
                    ))
                }
            }
        }

        result
    }

    fn count(&self, cuboid: &Cuboid) -> usize {
        return (cuboid.max.x - cuboid.min.x + 1).abs() as usize
            * (cuboid.max.y - cuboid.min.y + 1).abs() as usize
            * (cuboid.max.z - cuboid.min.z + 1).abs() as usize;
    }

    fn resize(&self, cuboid: &Cuboid) -> Cuboid {
        let min_x = max(cuboid.min.x, -50);
        let min_y = max(cuboid.min.y, -50);
        let min_z = max(cuboid.min.z, -50);
        let max_x = min(cuboid.max.x, 50);
        let max_y = min(cuboid.max.y, 50);
        let max_z = min(cuboid.max.z, 50);

        Cuboid::new(
            Point3D::new(min_x, min_y, min_z),
            Point3D::new(max_x, max_y, max_z),
        )
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let rules = self.parse(input);

        let mut result = HashSet::new();
        for rule in rules {
            let resized = self.resize(&rule.cuboid);
            for item in self.generate(&resized) {
                if rule.status {
                    result.insert(item);
                } else {
                    result.remove(&item);
                }
            }
        }

        Ok(Box::new(result.len()))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(1))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day22::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day22_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day22_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
