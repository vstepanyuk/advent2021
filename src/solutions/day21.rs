use memoize::memoize;

use std::cmp::max;
use std::fmt::Display;

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Default)]
struct Dice {
    current: usize,
    count: usize,
}

impl Dice {
    fn next(&mut self) -> usize {
        self.count += 1;
        self.current = self.current % 100 + 1;
        self.current
    }
}

impl DaySolution {
    fn parse(&self, input: Option<String>) -> (usize, usize) {
        let input = input
            .unwrap()
            .replace("Player 1 starting position: ", "")
            .replace("Player 2 starting position: ", "");

        let (player1, player2) = input.split_once('\n').unwrap();
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

type Player = (usize, usize);

fn play(dice: &mut Dice, playing: Player, waiting: Player) -> usize {
    let mut playing = playing;

    playing.1 = (playing.1 + dice.next() + dice.next() + dice.next() - 1) % 10 + 1;
    playing.0 += playing.1;

    if playing.0 >= 1000 {
        return waiting.0 * dice.count;
    }

    play(dice, waiting, playing)
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (player1, player2) = self.parse(input);

        Ok(Box::new(play(
            &mut Dice::default(),
            (0, player1),
            (0, player2),
        )))
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
