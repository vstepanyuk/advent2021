use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Default)]
pub struct DaySolution;

fn is_small(s: &str) -> bool {
    s.chars().all(|ch| ('a'..'z').contains(&ch))
}

fn can_reach(
    g: &HashMap<String, Vec<String>>,
    visited: HashMap<String, i32>,
    start: String,
    path: Vec<String>,
    result: &mut Vec<Vec<String>>,
) {
    let to_nodes = g.get(&start).unwrap();

    let mut visited = visited.clone();
    let mut path = path.clone();
    path.push(start.clone());

    for node in to_nodes {
        if node == "end" {
            path.push("end".to_string());
            println!("{:?}", path.join(","));
            result.push(path.clone());
            continue;
        }

        let c = path.iter().filter(|&n| n == node).count();
        if is_small(node) && c > 0 {
            println!("-- {:?} small {} {}", path, node, c);

            continue;
        }

        can_reach(g, visited.clone(), node.clone(), path.clone(), result);
    }
}

fn can_reach2(
    g: &HashMap<String, Vec<String>>,
    visited: HashMap<String, i32>,
    start: String,
    path: Vec<String>,
    result: &mut Vec<Vec<String>>,
    can: bool,
) {
    let to_nodes = g.get(&start).unwrap();

    let mut visited = visited.clone();
    let mut path = path.clone();
    path.push(start.clone());

    for node in to_nodes {
        if node == "start" {
            continue;
        }

        if node == "end" {
            // println!("{:?}", path.join(","));
            path.push("end".to_string());
            result.push(path.clone());
            continue;
        }

        let mut c = path.iter().filter(|&n| n == node).count();

        if is_small(node) {
            if !can && c > 0 {
                continue;
            }
        }

        if !is_small(node) {
            c = 0;
        }

        // println!("/// {:?} {} {:?} {}", path.join(" -> "), node, can, c);

        can_reach2(
            g,
            visited.clone(),
            node.clone(),
            path.clone(),
            result,
            (can && c < 1),
        );
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);

        let mut hashmap = HashMap::<String, Vec<String>>::new();

        for line in lines {
            let (from, to) = line.split_once('-').unwrap();

            let from = from.to_string(); //.parse::<Node>().unwrap();
            let to = to.to_string(); //.parse::<Node>().unwrap();

            hashmap
                .entry(from.clone())
                .or_insert(vec![])
                .push(to.clone());

            if from != "start" {
                hashmap.entry(to).or_insert(vec![]).push(from);
            }
        }
        hashmap.insert("end".to_string(), vec![]);
        println!("{:?}", hashmap);

        let mut result: Vec<Vec<String>> = vec![];
        can_reach(
            &hashmap,
            HashMap::new(),
            "start".to_string(),
            vec![],
            &mut result,
        );

        Ok(Box::new(result.len()))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let lines = parse_lines::<String>(input);

        let mut hashmap = HashMap::<String, Vec<String>>::new();

        for line in lines {
            let (from, to) = line.split_once('-').unwrap();

            let from = from.to_string(); //.parse::<Node>().unwrap();
            let to = to.to_string(); //.parse::<Node>().unwrap();

            hashmap
                .entry(from.clone())
                .or_insert(vec![])
                .push(to.clone());

            if from != "start" {
                hashmap.entry(to).or_insert(vec![]).push(from);
            }
        }
        hashmap.insert("end".to_string(), vec![]);
        println!("{:?}", hashmap);

        let mut result: Vec<Vec<String>> = vec![];
        can_reach2(
            &hashmap,
            HashMap::new(),
            "start".to_string(),
            vec![],
            &mut result,
            true,
        );

        Ok(Box::new(result.len()))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day12::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day12_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day12_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
