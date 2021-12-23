use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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

type Room = (char, char);
type Room2 = (char, char, char, char);

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct State {
    rooms: [Room; 4],
    hallway: [char; 11],
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct State2 {
    rooms: [Room2; 4],
    hallway: [char; 11],
}

impl Display for State {
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
        writeln!(f, "  #########")?;

        Ok(())
    }
}

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
    fn solve1(&self, start_state: [Room; 4]) {
        let winning_room = [('A', 'A'), ('B', 'B'), ('C', 'C'), ('D', 'D')];

        let state = State {
            rooms: start_state,
            hallway: ['.'; 11], // Empty
        };

        let result = dijkstra(
            &state,
            |state| {
                // println!("Current state\n{}", state);
                let mut new_states: Vec<(State, usize)> = vec![];

                assert_eq!(state.hallway[2], '.');
                assert_eq!(state.hallway[4], '.');
                assert_eq!(state.hallway[6], '.');
                assert_eq!(state.hallway[8], '.');

                for (room_index, room) in state.rooms.iter().enumerate() {
                    if winning_room[room_index] == *room {
                        // println!("Winning room\n{}", state);

                        continue;
                    }

                    let (am1, am2) = room;

                    if *am1 == '.' && *am2 == '.' {
                        // println!("Empty room\n{}", state);
                        continue;
                    }

                    if *am1 == '.' {
                        let mut new_state = state.clone();
                        new_state.rooms[room_index] = (*am2, '.');

                        let cost = *COSTS.get(am2).unwrap();
                        new_states.push((new_state, cost));

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

                        let mut new_state = state.clone();

                        new_state.hallway[move_to] = *am1;
                        new_state.rooms[room_index].0 = '.';

                        let distance = self.distance(room_index, move_to);
                        let cost = *COSTS.get(am1).unwrap();

                        new_states.push((new_state, cost * distance));
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

                    if is_debug {
                        println!("DEBUG {} {}", can_move, am);
                        println!("{} {}", room_hallway_index, move_from);
                    }

                    if !can_move {
                        continue;
                    }

                    let (am1, am2) = state.rooms[room_index];
                    if am1 != '.' {
                        if am2 == '.' {
                            let mut new_state = state.clone();
                            new_state.rooms[room_index] = ('.', am1);

                            let cost = *COSTS.get(&am1).unwrap();
                            new_states.push((new_state, cost));
                        }

                        continue;
                    }

                    let mut new_state = state.clone();
                    new_state.rooms[room_index] = (am, am2);
                    new_state.hallway[move_from] = '.';

                    let distance = self.distance(room_index, move_from);
                    let cost = *COSTS.get(&am).unwrap();
                    new_states.push((new_state, cost * distance));
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

    fn solve2(&self, start_state: [Room2; 4]) {
        let mut costs = HashMap::new();
        costs.insert('A', 1usize);
        costs.insert('B', 10usize);
        costs.insert('C', 100usize);
        costs.insert('D', 1000usize);

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
                // println!("Current state\n{}", state);
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
        match (room_index, position) {
            (0, 0) => 3,
            (0, 1) => 2,
            (0, 3) => 2,
            (0, 5) => 4,
            (0, 7) => 6,
            (0, 9) => 8,
            (0, 10) => 9,
            (1, 0) => 5,
            (1, 1) => 4,
            (1, 3) => 2,
            (1, 5) => 2,
            (1, 7) => 4,
            (1, 9) => 6,
            (1, 10) => 7,
            (2, 0) => 7,
            (2, 1) => 6,
            (2, 3) => 4,
            (2, 5) => 2,
            (2, 7) => 2,
            (2, 9) => 4,
            (2, 10) => 5,
            (3, 0) => 9,
            (3, 1) => 8,
            (3, 3) => 6,
            (3, 5) => 4,
            (3, 7) => 2,
            (3, 9) => 2,
            (3, 10) => 3,
            _ => unreachable!(),
        }
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        self.solve1([('A', 'D'), ('C', 'D'), ('B', 'B'), ('A', 'C')]);

        Ok(Box::new(1))
    }

    fn part_2(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        self.solve2([
            ('A', 'D', 'D', 'D'),
            ('C', 'C', 'B', 'D'),
            ('B', 'B', 'A', 'B'),
            ('A', 'A', 'C', 'C'),
        ]);

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
