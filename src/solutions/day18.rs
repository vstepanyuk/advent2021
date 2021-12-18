use crate::solutions::{Result, Solution};
use nom::branch::alt;
use nom::bytes::streaming::take_while1;
use nom::character::complete::char;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{combinator::map_res, IResult};
use std::collections::VecDeque;
use std::fmt::Display;
use trees::{Node, Tree, TreeWalk};

#[derive(Debug, PartialEq, Eq)]
enum SnailFish {
    Pair1(i32, i32),
    Pair2(i32, Box<SnailFish>),
    Pair3(Box<SnailFish>, i32),
    Pair4(Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    fn tree(&self) -> Tree<(i32, usize)> {
        self._tree(0)
    }

    fn _tree(&self, depth: usize) -> Tree<(i32, usize)> {
        let mut tree = Tree::new((-1, depth));

        match self {
            SnailFish::Pair1(a, b) => {
                tree.root_mut().push_back(Tree::new((*a, depth)));
                tree.root_mut().push_back(Tree::new((*b, depth)));
            }
            SnailFish::Pair2(a, b) => {
                tree.root_mut().push_back(Tree::new((*a, depth)));
                tree.root_mut().push_back(b._tree(depth + 1));
            }
            SnailFish::Pair3(a, b) => {
                tree.root_mut().push_back(a._tree(depth + 1));
                tree.root_mut().push_back(Tree::new((*b, depth)));
            }
            SnailFish::Pair4(a, b) => {
                tree.root_mut().push_back(a._tree(depth + 1));
                tree.root_mut().push_back(b._tree(depth + 1));
            }
        }

        tree
    }

    // fn add_to_leftmost(self, value: i32) -> Self {
    //     match self {
    //         Self::Value(n) => Self::Value(n + value),
    //         Self::Pair2(l, r) => Self::Pair2(Box::new(l.add_to_leftmost(val)), r),
    //     }
    // }
    //
    // fn add_to_rightmost(self, val: u64) -> Self {
    //     match self {
    //         Self::Regular(n) => Number::Regular(n + val),
    //         Number::Pair(l, r) => Number::Pair(l, Box::new(r.add_to_rightmost(val))),
    //     }
    // }

    fn reduce(&self, level: usize) -> Self {
        println!("{:?} {:?}", level, self);
        match self {
            Self::Pair1(a, b) => SnailFish::Pair1(*a, *b),
            Self::Pair2(a, b) if level >= 4 => {
                if let Self::Pair1(c, d) = **b {
                    SnailFish::Pair1(a + c, 0)
                } else {
                    SnailFish::Pair2(*a, Box::new(b.reduce(level + 1)))
                }
            }
            Self::Pair2(a, b) => SnailFish::Pair2(*a, Box::new(b.reduce(level + 1))),
            Self::Pair3(a, b) if level >= 4 => {
                if let Self::Pair1(c, d) = **a {
                    SnailFish::Pair1(0, d + b)
                } else {
                    SnailFish::Pair3(Box::new(a.reduce(level + 1)), *b)
                }
            }
            Self::Pair3(a, b) => SnailFish::Pair3(Box::new(a.reduce(level + 1)), *b),
            Self::Pair4(a, b) => {
                SnailFish::Pair4(Box::new(a.reduce(level + 1)), Box::new(b.reduce(level + 1)))
            }
        }
    }

    fn explode(&self) {
        let mut tree = self.tree();
        let mut walk = TreeWalk::from(tree);

        while let Some(visit) = walk.get() {
            let node = visit.node();
            let (data, depth) = *node.data();

            if data == -1 && depth == 4 {
                let left = *walk.get().unwrap().node().data();
                walk.forward();
                let right = *walk.get().unwrap().node().data();
                walk.forward();

                // println!("LEFT: {:?}, RIGHT: {:?}", *left, *right);
                break;
            }
            // visit.node()
            println!("{} {:?} ({})", " ".repeat(depth), data, depth);

            walk.forward();
        }

        // self._explode(&mut tree, 0);
    }

    fn _explode(&self, root: &mut Tree<i32>, level: usize) {
        let mut iter = root.iter();
        let left = iter.next();
        let right = iter.next();
    }

    fn to_string(&self) -> String {
        match self {
            SnailFish::Pair1(a, b) => format!("[{},{}]", a, b),
            SnailFish::Pair2(a, b) => format!("[{},{}]", a, b.to_string()),
            SnailFish::Pair3(a, b) => format!("[{},{}]", a.to_string(), b),
            SnailFish::Pair4(a, b) => format!("[{},{}]", a.to_string(), b.to_string()),
        }
    }

    fn reduce_tree(&self, tree: &Tree<i32>) {
        for node in tree.iter() {
            println!("{:?} {:?}", node.data(), node.node_count());
        }
    }
}

impl Default for SnailFish {
    fn default() -> Self {
        unimplemented!()
    }
}

trait ToSnailfish {
    fn to_snailfish(&self) -> SnailFish;
}

impl ToSnailfish for str {
    fn to_snailfish(&self) -> SnailFish {
        Parser::parse(&self).unwrap()
    }
}

#[derive(Default)]
pub struct DaySolution;

struct Parser;
impl Parser {
    fn parse_int(input: &str) -> std::result::Result<i32, std::num::ParseIntError> {
        i32::from_str_radix(input, 10)
    }

    fn pair1(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::value, char(','), Self::value),
                char(']'),
            ),
        )(input)?;

        Ok((input, SnailFish::Pair1(a, b)))
    }

    fn pair2(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::value, char(','), Self::snailfish),
                char(']'),
            ),
        )(input)?;

        Ok((input, SnailFish::Pair2(a, Box::new(b))))
    }

    fn pair3(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::snailfish, char(','), Self::value),
                char(']'),
            ),
        )(input)?;

        Ok((input, SnailFish::Pair3(Box::new(a), b)))
    }

    fn pair4(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::snailfish, char(','), Self::snailfish),
                char(']'),
            ),
        )(input)?;

        Ok((input, SnailFish::Pair4(Box::new(a), Box::new(b))))
    }

    fn value(input: &str) -> IResult<&str, i32> {
        map_res(take_while1(|ch: char| ch.is_digit(10)), Self::parse_int)(input)
    }

    fn snailfish(input: &str) -> IResult<&str, SnailFish> {
        alt((Parser::pair1, Parser::pair2, Parser::pair3, Parser::pair4))(input)
    }

    fn parse(input: &str) -> Option<SnailFish> {
        Self::snailfish(&input).map(|(_, snailfish)| snailfish).ok()
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        // let f = Parser::parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        // f.debug();
        // f.reduce(1).debug();

        let f = Parser::parse("[[[[[9,8],1],2],3],4]").unwrap();
        // println!("{:?}", f.reduce(1).to_string());
        f.explode();

        // let f = Parser::parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        // println!("{:?}", f.reduce(1).to_string());
        //
        // let f = Parser::parse("[[6,[5,[4,[3,2]]]],1]").unwrap();
        // println!("{:?}", f.reduce(1).to_string());

        // let f = Parser::parse("[[6,[5,[4,[3,2]]]],1]").unwrap();
        // f.explode();
        // // println!("{:?}", f);
        // let tree = f.tree();
        // println!("{:?}", tree.to_string());
        //
        // f.reduce_tree(&tree);

        Ok(Box::new(1))
    }

    fn part_2(&mut self, _input: Option<String>) -> Result<Box<dyn Display>> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day18::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day18_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day18_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
