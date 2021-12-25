use crate::matrix::Matrix;
use crate::solutions::{Result, Solution};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

type Sea = Matrix<char>;

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let mut sea = Sea::from(&input.unwrap()).unwrap();
        let mut count = 0;

        loop {
            let mut moves = 0;

            // Move >
            let mut moved = HashSet::new();
            sea.iter()
                .filter(|(&c, _)| c == '>')
                .map(|(_, pos)| pos)
                .collect::<Vec<_>>()
                .iter()
                .for_each(|(x, y)| {
                    let x = *x as i32;
                    let y = *y as i32;

                    let new_x = match (sea.get(x + 1, y), sea.get(0, y)) {
                        (Some('.'), _) if !moved.contains(&(x + 1, y)) => Some(x + 1),
                        (None, Some('.')) if !moved.contains(&(0, y)) => Some(0),
                        _ => None,
                    };

                    if let Some(new_x) = new_x {
                        sea.set(x, y, '.');
                        sea.set(new_x, y, '>');
                        moved.insert((x, y));
                        moves += 1;
                    }
                });

            // Move v
            let mut moved = HashSet::new();
            sea.iter()
                .filter(|(&c, _)| c == 'v')
                .map(|(_, pos)| pos)
                .collect::<Vec<_>>()
                .iter()
                .for_each(|(x, y)| {
                    let x = *x as i32;
                    let y = *y as i32;

                    let new_y = match (sea.get(x, y + 1), sea.get(x, 0)) {
                        (Some('.'), _) if !moved.contains(&(x, y + 1)) => Some(y + 1),
                        (None, Some('.')) if !moved.contains(&(x, 0)) => Some(0),
                        _ => None,
                    };

                    if let Some(new_y) = new_y {
                        sea.set(x, y, '.');
                        sea.set(x, new_y, 'v');
                        moved.insert((x, y));
                        moves += 1;
                    }
                });

            if moves == 0 {
                break;
            }

            count += 1;
        }

        Ok(Box::new(count + 1))
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
