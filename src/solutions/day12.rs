use crate::helpers::parse_lines;
use crate::solutions::{Result, Solution};
use std::collections::HashMap;
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

    fn solve(
        &self,
        graph: &HashMap<String, Vec<String>>,
        start: &str,
        path: &[String],
        is_part2: bool,
    ) -> usize {
        let to_nodes = graph.get(start).unwrap();

        let mut path = path.to_owned();
        path.push(start.to_string());

        let mut count = 0;
        for to_node in to_nodes {
            count += match to_node.as_str() {
                "start" => 0,
                "end" => 1,
                _ if to_node.is_lowercase() && !is_part2 && path.occurrences(to_node) > 0 => 0,
                _ => self.solve(
                    graph,
                    to_node,
                    &path,
                    is_part2 && (!to_node.is_lowercase() || path.occurrences(to_node) < 1),
                ),
            };
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
        let count = self.solve(&graph, &"start".to_string(), &[], false);
        Ok(Box::new(count))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let graph = self.parse(input);
        let count = self.solve(&graph, &"start".to_string(), &[], true);
        Ok(Box::new(count))
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
