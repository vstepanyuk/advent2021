use crate::solutions::{Result, Solution};
use euclid::{Box3D, Point3D};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

type CuboidBox = Box3D<isize, ()>;

struct Cuboid {
    r#box: CuboidBox,
    empty: Vec<Cuboid>,
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Self> {
        if let Some(r#box) = self.r#box.intersection(&other.r#box) {
            Some(Cuboid {
                r#box,
                empty: vec![],
            })
        } else if self.r#box.contains_box(&other.r#box) {
            Some(Cuboid {
                r#box: Box3D::new(other.r#box.min, other.r#box.max),
                empty: vec![],
            })
        } else {
            None
        }
    }

    fn subtract(&mut self, other: &Cuboid) {
        if let Some(cuboid) = self.intersection(other) {
            self.empty.iter_mut().for_each(|empty| {
                empty.subtract(&cuboid);
            });
            self.empty.push(cuboid);
        }
    }

    fn count(&self) -> isize {
        let count = self.count_box(&self.r#box);
        count - self.empty.iter().map(Cuboid::count).sum::<isize>()
    }

    fn count_box(&self, r#box: &CuboidBox) -> isize {
        (r#box.max.x - r#box.min.x + 1)
            * (r#box.max.y - r#box.min.y + 1)
            * (r#box.max.z - r#box.min.z + 1)
    }
}

#[derive(Debug)]
struct Rule {
    cuboid: CuboidBox,
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

                    (mn, mx)
                })
                .collect::<Vec<_>>();

            let cuboid = CuboidBox::new(
                Point3D::new(result[0].0, result[1].0, result[2].0),
                Point3D::new(result[0].1, result[1].1, result[2].1),
            );

            rules.push(Rule { cuboid, status })
        }

        rules
    }

    fn generate(&self, cuboid: &CuboidBox) -> Vec<CuboidBox> {
        let mut result = vec![];

        for x in cuboid.min.x..=cuboid.max.x {
            for y in cuboid.min.y..=cuboid.max.y {
                for z in cuboid.min.z..=cuboid.max.z {
                    result.push(CuboidBox::new(Point3D::new(x, y, z), Point3D::new(x, y, z)))
                }
            }
        }

        result
    }

    fn resize(&self, cuboid: &CuboidBox) -> CuboidBox {
        let min_x = max(cuboid.min.x, -50);
        let min_y = max(cuboid.min.y, -50);
        let min_z = max(cuboid.min.z, -50);
        let max_x = min(cuboid.max.x, 50);
        let max_y = min(cuboid.max.y, 50);
        let max_z = min(cuboid.max.z, 50);

        CuboidBox::new(
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
        let rules = self.parse(input);

        let mut cuboids: Vec<Cuboid> = vec![];

        for rule in rules {
            let cuboid = Cuboid {
                r#box: rule.cuboid,
                empty: vec![],
            };

            for c in cuboids.iter_mut() {
                c.subtract(&cuboid);
            }

            if rule.status {
                cuboids.push(cuboid);
            }
        }

        Ok(Box::new(cuboids.iter().map(Cuboid::count).sum::<isize>()))
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
