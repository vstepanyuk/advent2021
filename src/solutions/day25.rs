use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

type Sea = Matrix<char>;

impl DaySolution {
    fn r#move(&self, sea: &mut Sea, cucumber: char) -> bool {
        let next = if cucumber == '>' {
            (1, 0, 0, 1)
        } else {
            (0, 1, 1, 0)
        };

        let mut moved = HashSet::new();
        sea.iter()
            .filter(|(&c, _)| c == cucumber)
            .map(|(_, pos)| pos)
            .collect::<Vec<_>>()
            .iter()
            .fold(false, |result, (x, y)| {
                let x = *x as i32;
                let y = *y as i32;

                let new_pos = match (
                    sea.get(x + next.0, y + next.1),
                    sea.get(x * next.2, y * next.3),
                ) {
                    (Some('.'), _) if !moved.contains(&(x + next.0, y + next.1)) => {
                        Some((x + next.0, y + next.1))
                    }
                    (None, Some('.')) if !moved.contains(&(x * next.2, y * next.3)) => {
                        Some((x * next.2, y * next.3))
                    }
                    _ => None,
                };

                if let Some((new_x, new_y)) = new_pos {
                    sea.set(x, y, '.');
                    sea.set(new_x, new_y, cucumber);
                    moved.insert((x, y));
                    return true;
                }

                result
            })
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut sea = Sea::from(&input.unwrap()).unwrap();

        Ok(Box::new(
            std::iter::repeat(1)
                .take_while(|_| self.r#move(&mut sea, '>') | self.r#move(&mut sea, 'v'))
                .count()
                + 1,
        ))
    }

    fn part_2(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new("NO PART 2"))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day25::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day25_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day25_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
