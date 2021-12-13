use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};

#[derive(Default)]
pub struct DaySolution;

type Node = (String, bool);

impl DaySolution {
    fn node_build(&self, name: &str) -> Node {
        (name.to_string(), name.chars().all(|ch| ch.is_lowercase()))
    }

    fn parse(&self, input: Option<String>) -> HashMap<Node, Vec<Node>> {
        let lines = parse_lines::<String>(input);
        let mut graph = HashMap::<_, Vec<_>>::new();

        for line in lines {
            let (from, to) = line.split_once('-').unwrap();

            let from = self.node_build(from);
            let to = self.node_build(to);

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
        queue.push_back((
            HashSet::new(),
            graph.get(&self.node_build("start")).unwrap(),
            count_twice,
        ));

        while let Some((path, to_nodes, count_twice)) = queue.pop_front() {
            count += to_nodes
                .iter()
                .filter(|&node| match node.0.as_str() {
                    "end" => true,
                    "start" => false,
                    _ if node.1 && !count_twice && path.contains(node) => false,
                    _ => {
                        let mut new_path = path.clone();
                        new_path.insert(node.clone());

                        queue.push_back((
                            new_path,
                            graph.get(node).unwrap(),
                            count_twice && (!node.1 || !path.contains(node)),
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
