use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use pathfinding::prelude::dijkstra;

use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

lazy_static! {
    static ref COSTS: HashMap::<char, usize> = {
        let mut costs = HashMap::new();
        costs.insert('A', 1usize);
        costs.insert('B', 10usize);
        costs.insert('C', 100usize);
        costs.insert('D', 1000usize);
        costs
    };
}

type Amphipod = char;
type Hallway = [Amphipod; 11];
type Room = Vec<Amphipod>;
type Rooms = [Room; 4];
type State = (Rooms, Hallway);

const AMPHIPOD_HALLWAY_MOVES: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl DaySolution {
    fn solve(&self, input: Option<String>, animate: bool) -> String {
        let initial_state = self.parse(input);
        let room_size = initial_state[0].len();
        let final_state = [
            vec!['A'; room_size],
            vec!['B'; room_size],
            vec!['C'; room_size],
            vec!['D'; room_size],
        ];

        let result = dijkstra(
            &(initial_state, ['.'; 11]),
            |state| {
                let (rooms, hallway) = state;
                let mut states: Vec<(State, usize)> = vec![];

                // From rooms
                for (index, room) in rooms.iter().enumerate() {
                    if room == &final_state[index] {
                        continue;
                    }

                    // Room is empty
                    if room.iter().all(|a| *a == '.') {
                        continue;
                    }

                    let room_index = (index + 1) * 2;
                    for move_to in AMPHIPOD_HALLWAY_MOVES {
                        let range = min(move_to, room_index)..=max(move_to, room_index);

                        if hallway[range].iter().any(|a| *a != '.') {
                            // Blocked
                            continue;
                        }

                        let (mut rooms, mut hallway) = state.clone();

                        let distance = rooms[index]
                            .iter()
                            .position(|amphipod| *amphipod != '.')
                            .unwrap();

                        let amphipod = rooms[index][distance];
                        if amphipod == final_state[index][0]
                            && rooms[index][distance..].iter().all_equal()
                        {
                            continue;
                        }

                        let cost = *COSTS.get(&amphipod).unwrap()
                            * (distance + 1 + self.distance(index, move_to));

                        hallway[move_to] = amphipod;
                        rooms[index][distance] = '.';

                        states.push(((rooms, hallway), cost));
                    }
                }

                // To rooms
                for move_from in AMPHIPOD_HALLWAY_MOVES {
                    let amphipod = hallway[move_from];

                    if amphipod == '.' {
                        continue;
                    }
                    let index = (amphipod as u8 - b'A') as usize;

                    if rooms[index].iter().any(|a| !(*a == amphipod || *a == '.')) {
                        continue;
                    }

                    if rooms[index][0] != '.' {
                        // Room is full
                        continue;
                    }

                    let room_index = (index + 1) * 2;

                    let range = min(room_index, move_from) + 1..=max(room_index, move_from) - 1;
                    if hallway[range].iter().any(|c| *c != '.') {
                        // Blocked
                        continue;
                    }

                    let (mut rooms, mut hallway) = state.clone();
                    hallway[move_from] = '.';

                    let distance = rooms[index]
                        .iter()
                        .position(|a| *a != '.')
                        .unwrap_or(rooms[index].len())
                        - 1;

                    let cost = *COSTS.get(&amphipod).unwrap()
                        * (distance + 1 + self.distance(index, move_from));

                    rooms[index][distance] = amphipod;

                    states.push(((rooms, hallway), cost));
                }

                states
            },
            |(rooms, _)| rooms == &final_state,
        );

        if let Some((path, cost)) = result {
            if animate {
                for _ in 0..100 {
                    println!();
                }
                for state in path {
                    print!("\x1B[2J");
                    println!("{}", self.draw(&state));
                    sleep(Duration::from_millis(500));
                }
                println!();
            }

            return cost.to_string();
        }

        "No solution".to_string()
    }

    fn distance(&self, room_index: usize, position: usize) -> usize {
        (position as i32 - (room_index as i32 + 1) * 2).abs() as usize
    }

    fn parse(&self, input: Option<String>) -> Rooms {
        let input = input.unwrap();
        let lines = input
            .lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = lines.len() - 3;

        let mut rooms = [
            vec!['.'; size],
            vec!['.'; size],
            vec!['.'; size],
            vec!['.'; size],
        ];

        for i in 0..size {
            for (r, room) in rooms.iter_mut().enumerate() {
                room[i] = lines[i + 2][(r + 1) * 2 + 1];
            }
        }

        rooms
    }

    fn draw(&self, state: &State) -> String {
        let (rooms, hallway) = state;
        let mut output = vec![];
        output.push("╭───────────╮".to_string());
        output.push(format!("│{}│", hallway.iter().collect::<String>()));
        output.push("╰─╮ ┬ ┬ ┬ ╭─╯".to_string());
        for level in 0..rooms[0].len() {
            output.push(format!(
                "  │{}│{}│{}│{}│",
                rooms[0][level], rooms[1][level], rooms[2][level], rooms[3][level]
            ));
        }
        output.push("  ╰─┴─┴─┴─╯".to_string());
        output.join("\n")
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, true)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, true)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day23::DaySolution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day23_demo.txt");
        let result = DaySolution::default().solve(Some(input.to_string()), false);

        assert_eq!("12521", result);

        let input = include_str!("../../inputs/day23.txt");
        let result = DaySolution::default().solve(Some(input.to_string()), false);

        assert_eq!("18195", result);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day23_demo_2.txt");
        let result = DaySolution::default().solve(Some(input.to_string()), false);

        assert_eq!("44169", result);

        let input = include_str!("../../inputs/day23_2.txt");
        let result = DaySolution::default().solve(Some(input.to_string()), false);

        assert_eq!("50265", result);
    }
}
