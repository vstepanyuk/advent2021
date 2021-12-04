use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, Default, Copy, Clone)]
struct BoardValue(u8, bool);
type Board = Vec<BoardValue>;

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
                .map(|v| BoardValue(v, false))
                .collect::<Vec<BoardValue>>();

            board.extend(row_nums)
        }

        (nums, boards)
    }

    fn play_board(&self, value: u8, board: &mut Board) {
        for item in board.iter_mut() {
            if value == item.0 {
                item.1 = true;
                break;
            }
        }
    }

    fn check_board(&self, board: &Board) -> bool {
        // rows
        for y in 0..5 {
            if (0..5).filter(|&x| board[x + y * 5].1).count() == 5 {
                return true;
            }

            if (0..5).filter(|&x| board[y + x * 5].1).count() == 5 {
                return true;
            }
        }

        false
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

        'outer: for num in nums {
            for board in boards.iter_mut() {
                self.play_board(num, board);

                if self.check_board(board) {
                    winner = Some((board.to_vec(), num));
                    break 'outer;
                }
            }
        }

        let (winner, num) = winner.unwrap();
        let sum = winner
            .into_iter()
            .filter(|&v| !v.1)
            .map(|v| v.0 as u32)
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
        'outer: for num in nums {
            for (index, board) in boards.iter_mut().enumerate() {
                self.play_board(num, board);

                if self.check_board(board) && !winners_idx.contains(&index) {
                    winners_idx.push(index);
                    winners.push((board.to_vec(), num));

                    if winners.len() == total_boards {
                        break 'outer;
                    }
                }
            }
        }

        let (winner, num) = winners.last().unwrap();
        let s1: u32 = winner.iter().filter(|v| !v.1).map(|v| v.0 as u32).sum();

        println!("{:?}", s1 * (*num as u32));

        Ok(())
    }
}
