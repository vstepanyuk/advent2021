use std::cmp::min;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ptr::hash;
use std::str::FromStr;

use itertools::iproduct;

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

impl DaySolution {
    fn parse(&self, input: Option<String>) -> (usize, usize) {
        let input = input
            .unwrap()
            .replace("Player 1 starting position: ", "")
            .replace("Player 2 starting position: ", "");

        let (player1, player2) = input.split_once("\n").unwrap();
        (
            player1.parse().unwrap_or_default(),
            player2.parse().unwrap_or_default(),
        )
    }

    fn dirac_dice() {}
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (mut player1, mut player2) = self.parse(input);
        let mut score1 = 0;
        let mut score2 = 0;
        let mut rolled = 0;
        let mut dice = 1;

        loop {
            for _ in 0..3 {
                player1 += dice;

                while player1 > 10 {
                    player1 -= 10;
                }

                dice += 1;
                if dice > 100 {
                    dice -= 100;
                }
            }
            rolled += 3;

            score1 += player1;
            if score1 >= 1000 {
                break;
            }

            for _ in 0..3 {
                player2 += dice;

                while player2 > 10 {
                    player2 -= 10;
                }

                dice += 1;
                if dice > 100 {
                    dice -= 100;
                }
            }
            rolled += 3;

            score2 += player2;
            if score2 >= 1000 {
                break;
            }
        }

        Ok(Box::new(min(score1, score2) * rolled))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(1))
    }
}

#[cfg(test)]
mod tests {
    use crate::day21::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day21_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("739785", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day21_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("", result.to_string());
    }
}
