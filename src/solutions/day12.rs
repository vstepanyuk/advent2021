use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Node {
    name: String,
    is_lowercase: bool,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_lowercase: name.chars().all(|ch| ch.is_lowercase()),
        }
    }
}

impl DaySolution {
    fn parse(&self, input: Option<String>) -> HashMap<Node, Vec<Node>> {
        let lines = parse_lines::<String>(input);
        let mut graph = HashMap::<_, Vec<_>>::new();

        for line in lines {
            let (from, to) = line.split_once('-').unwrap();

            let from = Node::new(from);
            let to = Node::new(to);

            graph
                .entry(from.clone())
                .or_insert_with(Vec::new)
                .push(to.clone());
            graph.entry(to).or_insert_with(Vec::new).push(from);
        }

        graph
    }

    fn solve(&self, graph: &HashMap<Node, Vec<Node>>, count_twice: bool) -> usize {
        let mut count = 0;
        let mut queue: VecDeque<(HashSet<_>, _, bool)> = VecDeque::new();
        queue.push_back((HashSet::new(), Node::new("start"), count_twice));

        while let Some((path, last_node, count_twice)) = queue.pop_front() {
            let to_nodes = graph.get(&last_node).unwrap();

            count += to_nodes
                .iter()
                .filter(|&node| match node.name.as_str() {
                    "end" => true,
                    "start" => false,
                    _ if node.is_lowercase && !count_twice && path.contains(node) => false,
                    _ => {
                        let mut new_path = path.clone();
                        new_path.insert(node.clone());

                        queue.push_back((
                            new_path,
                            node.to_owned(),
                            count_twice && (!node.is_lowercase || !path.contains(node)),
                        ));

                        false
                    }
                })
                .count();
        }

        count
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let graph = self.parse(input);
        Ok(Box::new(self.solve(&graph, false)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let graph = self.parse(input);
        Ok(Box::new(self.solve(&graph, true)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let input = include_str!("../../inputs/day12_demo.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("10", result.to_string());

        let input = include_str!("../../inputs/day12.txt");
        let result = DaySolution::default()
            .part_1(Some(input.to_string()))
            .unwrap();

        assert_eq!("4413", result.to_string());
    }

    #[test]
    fn part_2() {
        let input = include_str!("../../inputs/day12_demo.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("36", result.to_string());

        let input = include_str!("../../inputs/day12.txt");
        let result = DaySolution::default()
            .part_2(Some(input.to_string()))
            .unwrap();

        assert_eq!("118803", result.to_string());
    }
}
