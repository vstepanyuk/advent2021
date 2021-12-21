use memoize::memoize;
use std::cmp::{max, min};
use std::fmt::Display;

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
}

#[memoize]
fn dirac(score1: usize, score2: usize, pos1: usize, pos2: usize) -> (usize, usize) {
    if score1 >= 21 {
        return (1, 0);
    }
    if score2 >= 21 {
        return (0, 1);
    }

    let mut new_score_1 = 0;
    let mut new_score_2 = 0;

    for u1 in 1..=3 {
        for u2 in 1..=3 {
            for u3 in 1..=3 {
                let pos1 = (u1 + u2 + u3 + pos1 - 1) % 10 + 1;
                let result = dirac(score2, score1 + pos1, pos2, pos1);
                new_score_1 += result.1;
                new_score_2 += result.0;
            }
        }
    }

    (new_score_1, new_score_2)
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
        let (player1, player2) = self.parse(input);
        let result = dirac(0, 0, player1, player2);

        Ok(Box::new(max(result.0, result.1)))
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

        assert_eq!("444356092776315", result.to_string());
    }
}
