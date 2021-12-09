use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);

        let width = lines[0].len();
        let mut board: Vec<i32> = vec![];

        for line in lines {
            let value = line
                .chars()
                .map(|ch| (ch as u8 - b'0') as i32)
                .collect::<Vec<i32>>();

            board.extend(value);
        }
        let mut sum = 0;

        let mut len = board.len();

        let mut result = vec![];

        for x in 0..board.len() {
            let v = board.get(x).unwrap();
            let y = x / width;
            let xx = x % width;

            let l = board.get(xx - 1 + y * width).unwrap_or(&i32::MAX);
            let r = board.get(xx + 1 + y * width).unwrap_or(&i32::MAX);
            let t = board.get(xx + (y - 1) * width).unwrap_or(&i32::MAX);
            let b = board.get(xx + (y + 1) * width).unwrap_or(&i32::MAX);

            if v < l && v < r && v < t && v < b {
                result.push(*v + 1);
                println!("({}, {}) [{}] {} {} {} {}", x, y, v, l, r, t, b);
            }
        }
        println!("{:?}", result);

        sum = result.iter().sum::<i32>();

        Ok(Box::new(sum))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);

        let width = lines[0].len();
        let mut board: Vec<i32> = vec![];

        for line in lines {
            let value = line
                .chars()
                .map(|ch| (ch as u8 - b'0') as i32)
                .collect::<Vec<i32>>();

            board.extend(value);
        }
        let height = board.len() / width;
        println!("{} x {}", width, height);

        let mut points: Vec<(usize, usize)> = vec![];

        for x in 0..board.len() {
            let v = board.get(x).unwrap();
            let y = x / width;
            let xx = x % width;

            let l = board.get(xx - 1 + y * width).unwrap_or(&i32::MAX);
            let r = board.get(xx + 1 + y * width).unwrap_or(&i32::MAX);
            let t = board.get(xx + (y - 1) * width).unwrap_or(&i32::MAX);
            let b = board.get(xx + (y + 1) * width).unwrap_or(&i32::MAX);

            if v < l && v < r && v < t && v < b {
                points.push((xx, y));
            }
        }

        let mut counts: Vec<i32> = vec![];

        for point in points.iter() {
            let (x, y) = point;

            // let mut board = board.clone();
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

            queue.push_back((*x, *y));
            let mut count = 0;

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                let value = board.get(x + y * width).unwrap_or(&9);
                if *value == 9 {
                    continue;
                }

                board[x + y * width] = 9;
                count += 1;

                let l = board.get(x - 1 + y * width).unwrap_or(&9);
                let r = board.get(x + 1 + y * width).unwrap_or(&9);
                let t = board.get(x + (y - 1) * width).unwrap_or(&9);
                let b = board.get(x + (y + 1) * width).unwrap_or(&9);

                if (x > 0) {
                    queue.push_back((x - 1, y));
                }

                if (x + 1 < width) {
                    queue.push_back((x + 1, y));
                }
                if (y > 0) {
                    queue.push_back((x, (y - 1)));
                }

                if (y + 1 < height) {
                    queue.push_back((x, (y + 1)));
                }
            }

            counts.push(count);
        }

        counts.sort();
        counts.reverse();

        println!("{:?}", counts[0] * counts[1] * counts[2]);

        let mut sum = 0;

        Ok(Box::new(sum))
    }
}

fn debug(v: &Vec<i32>, width: usize) {
    let height = v.len() / width;
    for y in 0..height {
        for x in 0..width {
            print!("{} ", v[x + y * width]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::day9::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day9_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("15", result.to_string())
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day9_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("", result.to_string())
    }
}
