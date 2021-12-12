use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

trait StringExtras {
    fn is_lowercase(&self) -> bool;
}

impl StringExtras for str {
    fn is_lowercase(&self) -> bool {
        self.chars().all(|ch| ch.is_lowercase())
    }
}

trait VecExtras<T> {
    fn occurrences(&self, elem: &T) -> usize;
}

impl<T> VecExtras<T> for [T]
where
    T: PartialEq,
{
    fn occurrences(&self, elem: &T) -> usize {
        self.iter().filter(|&current| current == elem).count()
    }
}

impl DaySolution {
    fn parse(&self, input: Option<String>) -> HashMap<String, Vec<String>> {
        let lines = parse_lines::<String>(input);
        let mut graph = HashMap::<String, Vec<String>>::new();

        for line in lines {
            let (from, to) = line.split_once('-').unwrap();

            let from = from.to_string();
            let to = to.to_string();

            graph
                .entry(from.clone())
                .or_insert_with(Vec::new)
                .push(to.clone());
            graph.entry(to).or_insert_with(Vec::new).push(from);
        }
        graph.insert("end".to_string(), vec![]);
        graph
    }

    fn solve_queue(&self, graph: &HashMap<String, Vec<String>>, count_twice: bool) -> usize {
        let mut count = 0;
        let mut queue: VecDeque<(Vec<String>, bool)> = VecDeque::new();
        queue.push_back((vec!["start".to_string()], count_twice));

        while let Some((path, count_twice)) = queue.pop_front() {
            let last_node = path.last().unwrap();
            let to_nodes = graph.get(last_node).unwrap();

            for node in to_nodes.iter() {
                if node == "start" {
                    continue;
                }
                if node == "end" {
                    count += 1;
                    continue;
                }

                let is_lowercase = node.is_lowercase();
                let occurrences = path.occurrences(node);
                if is_lowercase && !count_twice && occurrences > 0 {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(node.to_owned());

                queue.push_back((
                    new_path,
                    count_twice && (!is_lowercase || path.occurrences(node) < 1),
                ));
            }
        }

        return count;
    }
}

impl Solution for DaySolution {
    fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let graph = self.parse(input);
        Ok(Box::new(self.solve_queue(&graph, false)))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let graph = self.parse(input);
        Ok(Box::new(self.solve_queue(&graph, true)))
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
