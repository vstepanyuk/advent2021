use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

type Room2 = (char, char, char, char);

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct State2 {
    rooms: [Room2; 4],
    hallway: [char; 11],
}

// impl Display for StateOld {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "╭───────────╮")?;
//         let hallway = self.hallway.iter().collect::<String>();
//         writeln!(f, "│{}│", hallway)?;
//         writeln!(f, "╰─╮ ┬ ┬ ┬ ╭─╯")?;
//         writeln!(
//             f,
//             "  │{}│{}│{}│{}│",
//             self.rooms[0].0, self.rooms[1].0, self.rooms[2].0, self.rooms[3].0,
//         )?;
//         writeln!(
//             f,
//             "  │{}│{}│{}│{}│",
//             self.rooms[0].1, self.rooms[1].1, self.rooms[2].1, self.rooms[3].1,
//         )?;
//         writeln!(f, "  ╰─┴─┴─┴─╯")?;
//         Ok(())
//     }
// }

impl Display for State2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        let hallway = self.hallway.iter().collect::<String>();
        writeln!(f, "#{}#", hallway)?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.rooms[0].0, self.rooms[1].0, self.rooms[2].0, self.rooms[3].0,
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.rooms[0].1, self.rooms[1].1, self.rooms[2].1, self.rooms[3].1,
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.rooms[0].2, self.rooms[1].2, self.rooms[2].2, self.rooms[3].2,
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.rooms[0].3, self.rooms[1].3, self.rooms[2].3, self.rooms[3].3,
        )?;
        writeln!(f, "  #########")?;

        Ok(())
    }
}

impl DaySolution {
    fn solve(&self, initial_rooms: Rooms, winning_rooms: Rooms) -> Option<(Vec<State>, usize)> {
        let result = dijkstra(
            &(initial_rooms, ['.'; 11]),
            |state| {
                println!("Current state");
                self.draw(state);

                let (rooms, hallway) = state;
                let mut states: Vec<(State, usize)> = vec![];

                // From rooms
                for (index, room) in rooms.iter().enumerate() {
                    if room == &winning_rooms[index] {
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
                        for (distance, amphipod) in rooms[index].iter_mut().enumerate() {
                            if *amphipod != '.' {
                                let cost = *COSTS.get(amphipod).unwrap()
                                    * (distance + self.distance(index, move_to));

                                hallway[move_to] = *amphipod;
                                *amphipod = '.';

                                states.push(((rooms, hallway), cost));
                                break;
                            }
                        }
                    }
                }

                // To rooms
                for move_from in AMPHIPOD_HALLWAY_MOVES {
                    let amphipod = hallway[move_from];
                    let index = (amphipod as u8 - b'A') as usize;

                    if amphipod == '.' {
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
                        * (distance + self.distance(index, move_from));

                    rooms[index][distance] = amphipod;

                    states.push(((rooms, hallway), cost));
                }

                states
            },
            |(rooms, _)| rooms == &winning_rooms,
        );

        result
    }

    fn solve2(&self, start_state: [Room2; 4]) {
        let winning_room = [
            ('A', 'A', 'A', 'A'),
            ('B', 'B', 'B', 'B'),
            ('C', 'C', 'C', 'C'),
            ('D', 'D', 'D', 'D'),
        ];

        let state = State2 {
            rooms: start_state,
            hallway: ['.'; 11], // Empty
        };

        let result = dijkstra(
            &state,
            |state| {
                println!("Current state\n{}", state);
                let mut new_states: Vec<(State2, usize)> = vec![];

                assert_eq!(state.hallway[2], '.');
                assert_eq!(state.hallway[4], '.');
                assert_eq!(state.hallway[6], '.');
                assert_eq!(state.hallway[8], '.');

                for (room_index, room) in state.rooms.iter().enumerate() {
                    if winning_room[room_index] == *room {
                        continue;
                    }

                    let (am1, am2, am3, am4) = room;

                    if *am1 == '.' && *am2 == '.' && *am3 == '.' && *am4 == '.' {
                        continue;
                    }

                    for move_to in [0, 1, 3, 5, 7, 9, 10] {
                        let room_hallway_index = (room_index + 1) * 2;
                        let can_move = if move_to < room_hallway_index {
                            (move_to..room_hallway_index).all(|index| state.hallway[index] == '.')
                        } else {
                            (room_hallway_index..=move_to).all(|index| state.hallway[index] == '.')
                        };

                        if !can_move {
                            continue;
                        }

                        match (am1, am2, am3, am4) {
                            ('.', '.', '.', am) => {
                                let mut new_state = state.clone();

                                new_state.hallway[move_to] = *am;
                                new_state.rooms[room_index].3 = '.';

                                let distance = self.distance(room_index, move_to) + 3;
                                let cost = *COSTS.get(am).unwrap();

                                new_states.push((new_state, cost * distance));
                            }
                            ('.', '.', am, _) => {
                                let mut new_state = state.clone();

                                new_state.hallway[move_to] = *am;
                                new_state.rooms[room_index].2 = '.';

                                let distance = self.distance(room_index, move_to) + 2;
                                let cost = *COSTS.get(am).unwrap();

                                new_states.push((new_state, cost * distance));
                            }
                            ('.', am, _, _) => {
                                let mut new_state = state.clone();

                                new_state.hallway[move_to] = *am;
                                new_state.rooms[room_index].1 = '.';

                                let distance = self.distance(room_index, move_to) + 1;
                                let cost = *COSTS.get(am).unwrap();

                                new_states.push((new_state, cost * distance));
                            }
                            (am, _, _, _) => {
                                let mut new_state = state.clone();

                                new_state.hallway[move_to] = *am;
                                new_state.rooms[room_index].0 = '.';

                                let distance = self.distance(room_index, move_to);
                                let cost = *COSTS.get(am).unwrap();

                                new_states.push((new_state, cost * distance));
                            }
                        }
                    }
                }

                let is_debug = false;

                if is_debug {
                    println!("{}", state);
                }

                // Moving from hallway
                for move_from in [0, 1, 3, 5, 7, 9, 10] {
                    let am = state.hallway[move_from];
                    if am == '.' {
                        continue;
                    }

                    let room_index = match am {
                        'A' => 0,
                        'B' => 1,
                        'C' => 2,
                        'D' => 3,
                        _ => unreachable!(),
                    };

                    let room_hallway_index = (room_index + 1) * 2;

                    let can_move = if move_from < room_hallway_index {
                        (move_from + 1..=room_hallway_index)
                            .all(|index| state.hallway[index] == '.')
                    } else {
                        (room_hallway_index..move_from).all(|index| state.hallway[index] == '.')
                    };

                    if !can_move {
                        continue;
                    }

                    let (am1, am2, am3, am4) = state.rooms[room_index];

                    match (am1, am2, am3, am4) {
                        ('.', '.', '.', '.') => {
                            let mut new_state = state.clone();
                            new_state.hallway[move_from] = '.';
                            new_state.rooms[room_index] = ('.', '.', '.', am);

                            let distance = self.distance(room_index, move_from) + 3;
                            let cost = *COSTS.get(&am).unwrap();
                            new_states.push((new_state, cost * distance));
                        }
                        ('.', '.', '.', _) => {
                            let mut new_state = state.clone();
                            new_state.hallway[move_from] = '.';
                            new_state.rooms[room_index] = ('.', '.', am, am4);

                            let distance = self.distance(room_index, move_from) + 2;
                            let cost = *COSTS.get(&am).unwrap();
                            new_states.push((new_state, cost * distance));
                        }
                        ('.', '.', _, _) => {
                            let mut new_state = state.clone();
                            new_state.hallway[move_from] = '.';
                            new_state.rooms[room_index] = ('.', am, am3, am4);

                            let distance = self.distance(room_index, move_from) + 1;
                            let cost = *COSTS.get(&am).unwrap();
                            new_states.push((new_state, cost * distance));
                        }
                        ('.', _, _, _) => {
                            let mut new_state = state.clone();
                            new_state.hallway[move_from] = '.';
                            new_state.rooms[room_index] = (am, am2, am3, am4);

                            let distance = self.distance(room_index, move_from);
                            let cost = *COSTS.get(&am).unwrap();
                            new_states.push((new_state, cost * distance));
                        }
                        _ => continue,
                    }

                    if is_debug {
                        println!("DEBUG {} {}", can_move, am);
                        println!("{} {}", room_hallway_index, move_from);
                    }
                }

                if is_debug {
                    println!("{:?}", new_states);
                }

                new_states
            },
            |state| state.rooms == winning_room,
        );

        let (path, cost) = result.unwrap();

        for p in path.iter() {
            println!("{}", p);
        }

        println!("{}", cost);
    }

    fn distance(&self, room_index: usize, position: usize) -> usize {
        return (position as i32 - (room_index as i32 + 1) * 2).abs() as usize + 1;
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
            for room in 0..4 {
                rooms[room][i] = lines[i + 2][(room + 1) * 2 + 1];
            }
        }

        rooms
    }

    fn draw(&self, state: &State) {
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
        println!("{}", output.join("\n"));
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let initial = self.parse(input);
        let winning = [vec!['A'; 2], vec!['B'; 2], vec!['C'; 2], vec!['D'; 2]];

        if let Some((path, cost)) = self.solve(initial, winning) {
            println!();
            for state in path {
                print!("\x1B[2J");
                self.draw(&state);
                sleep(Duration::from_millis(500));
            }
            println!();
            return Ok(Box::new(cost));
        }

        Ok(Box::new("No solution"))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let initial = self.parse(input);
        println!("{:?}", initial);

        let winning = [
            vec!['A', 'A'],
            vec!['B', 'B'],
            vec!['C', 'C'],
            vec!['D', 'D'],
        ];

        // self.solve2([
        //     ('A', 'D', 'D', 'D'),
        //     ('C', 'C', 'B', 'D'),
        //     ('B', 'B', 'A', 'B'),
        //     ('A', 'A', 'C', 'C'),
        // ]);

        let result = self.solve(initial, winning);
        println!("{:?}", result);

        Ok(Box::new(2))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day23::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day23_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day23_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
