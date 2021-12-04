use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, Default, Clone)]
struct BoardNum(u8, bool);
type Board = Vec<BoardNum>;

trait Playable {
    fn play_and_check(&mut self, value: u8) -> bool;
}

impl Playable for Board {
    fn play_and_check(&mut self, value: u8) -> bool {
        let mut found_index = None;
        for (index, mut num) in self.iter_mut().enumerate() {
            if value == num.0 {
                num.1 = true;
                found_index = Some(index);
                break;
            }
        }

        let index = match found_index {
            None => return false,
            Some(index) => index,
        };

        let row = index / 5;
        let col = index % 5;

        (0..5).filter(|&i| self[i + row * 5].1).count() == 5
            || (0..5).filter(|&i| self[col + i * 5].1).count() == 5
    }
}

impl DaySolution {
    fn parse(&self, lines: Vec<String>) -> (Vec<u8>, Vec<Board>) {
        let mut boards = vec![];
        let nums = lines
            .first()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut board = Board::default();
        for line in lines.iter().skip(2) {
            if line.is_empty() {
                boards.push(board.clone());
                board.clear();
                continue;
            }

            let row_nums = line
                .split(' ')
                .flat_map(|s| s.parse::<u8>().ok())
                .map(|v| BoardNum(v, false))
                .collect::<Vec<BoardNum>>();

            board.extend(row_nums)
        }
        boards.push(board);

        (nums, boards)
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<()> {
        let lines = parse_lines(input);
        let (nums, mut boards) = self.parse(lines);

        let mut winner: Option<(Board, u8)> = None;

        for num in nums {
            for board in boards.iter_mut() {
                if board.play_and_check(num) {
                    winner = Some((board.to_vec(), num));
                    break;
                }
            }

            if winner.is_some() {
                break;
            }
        }

        let (winner, num) = winner.unwrap();
        let sum = winner
            .into_iter()
            .filter(|num| !num.1)
            .map(|num| num.0 as u32)
            .sum::<u32>();

        println!("{:?}", sum * num as u32);

        Ok(())
    }

    fn part_2(&mut self, input: Option<String>) -> Result<()> {
        let lines = parse_lines(input);
        let (nums, mut boards) = self.parse(lines);

        let mut winners = vec![];
        let mut winners_idx = vec![];

        let total_boards = boards.len();
        for num in nums {
            for (index, board) in boards.iter_mut().enumerate() {
                if !winners_idx.contains(&index) && board.play_and_check(num) {
                    winners_idx.push(index);
                    winners.push((board.to_vec(), num));
                }
            }

            if winners.len() == total_boards {
                break;
            }
        }

        let (winner, num) = winners.last().unwrap();
        let sum: u32 = winner
            .iter()
            .filter(|num| !num.1)
            .map(|num| num.0 as u32)
            .sum();

        println!("{:?}", sum * (*num as u32));

        Ok(())
    }
}
