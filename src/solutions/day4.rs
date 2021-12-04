use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, Default, Clone)]
struct BoardNum(u8, bool);
type Board = Vec<BoardNum>;

trait Playable {
    fn play(&mut self, value: u8);
    fn is_winning(&self) -> bool;
}

impl Playable for Board {
    fn play(&mut self, value: u8) {
        for item in self.iter_mut() {
            if value == item.0 {
                item.1 = true;
                break;
            }
        }
    }

    fn is_winning(&self) -> bool {
        for y in 0..5 {
            if (0..5).filter(|&x| self[x + y * 5].1).count() == 5 {
                return true;
            }

            if (0..5).filter(|&x| self[y + x * 5].1).count() == 5 {
                return true;
            }
        }

        false
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
        let mut iter = lines.iter().skip(2);
        loop {
            let line = match iter.next() {
                Some(line) => line,
                None => {
                    boards.push(board.clone());
                    break;
                }
            };

            if line.is_empty() && !board.is_empty() {
                boards.push(board.clone());
                board = vec![];
                continue;
            }

            let row_nums = line
                .split(' ')
                .flat_map(|s| s.parse::<u8>().ok())
                .map(|v| BoardNum(v, false))
                .collect::<Vec<BoardNum>>();

            board.extend(row_nums)
        }

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
                board.play(num);

                if board.is_winning() {
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
                board.play(num);

                if board.is_winning() && !winners_idx.contains(&index) {
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
            .into_iter()
            .filter(|num| !num.1)
            .map(|num| num.0 as u32)
            .sum();

        println!("{:?}", sum * (*num as u32));

        Ok(())
    }
}
