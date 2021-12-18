use crate::solutions::{Result, Solution};
use nom::branch::alt;
use nom::bytes::streaming::take_while1;
use nom::character::complete::char;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{combinator::map_res, IResult};
use std::fmt::Display;
use trees::{Node, Tree};

#[derive(Debug, PartialEq, Eq)]
enum SnailFish {
    Pair1(i32, i32),
    Pair2(i32, Box<SnailFish>),
    Pair3(Box<SnailFish>, i32),
    Pair4(Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    fn tree(&self) -> Tree<i32> {
        let mut tree = Tree::new(0);

        match self {
            SnailFish::Pair1(a, b) => {
                tree.root_mut().push_back(Tree::new(*a));
                tree.root_mut().push_back(Tree::new(*b));
            }
            SnailFish::Pair2(a, b) => {
                tree.root_mut().push_back(Tree::new(*a));
                tree.root_mut().push_back(b.tree());
            }
            SnailFish::Pair3(a, b) => {
                tree.root_mut().push_back(a.tree());
                tree.root_mut().push_back(Tree::new(*b));
            }
            SnailFish::Pair4(a, b) => {
                tree.root_mut().push_back(a.tree());
                tree.root_mut().push_back(b.tree());
            }
        }

        tree

        // root.append()
        // match self {
        //     SnailFish::Pair1(a, b) => {
        //     }
        //     SnailFish::Pair2(_, _) => {}
        //     SnailFish::Pair3(_, _) => {}
        //     SnailFish::Pair4(_, _) => {}
        // }
    }

    fn reduce(&self, level: usize) -> SnailFish {
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

    fn debug(&self) {
        self.print(0);
    }

    fn print(&self, level: usize) {
        match self {
            SnailFish::Pair1(a, b) => print!("[{},{}]", a, b),
            SnailFish::Pair2(a, b) => {
                print!("[{},", a);
                b.print(level + 1);
                print!("]");
            }
            SnailFish::Pair3(a, b) => {
                print!("[");
                a.print(level + 1);
                print!(",{}]", b);
            }
            SnailFish::Pair4(a, b) => {
                print!("[");
                a.print(level + 1);
                print!(",");
                b.print(level + 1);
                print!("]");
            }
        }
        if level == 0 {
            println!();
        }
    }
}

impl Default for SnailFish {
    fn default() -> Self {
        unimplemented!()
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
        // println!("{:?}", Parser::parse("[1,2]"));
        // println!("{:?}", Parser::parse("[[1,2],3]"));
        // println!("{:?}", Parser::parse("[9,[8,7]]"));
        // println!("{:?}", Parser::parse("[[1,9],[8,5]]"));
        // println!("{:?}", Parser::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"));
        // println!(
        //     "{:?}",
        //     Parser::parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")
        // );

        println!("{:?}", Parser::parse("[1,2]").unwrap().tree().to_string());

        let f = Parser::parse("[[[[[9,8],1],2],3],4]").unwrap();
        // [[[[0,9],2],3],4]
        // f.debug();
        // f.reduce(1).debug();

        println!("{:?}", f.tree().to_string());

        let f = Parser::parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        // f.debug();
        // f.reduce(1).debug();

        let f = Parser::parse("[[6,[5,[4,[3,2]]]],1]").unwrap();
        // f.debug();
        // f.reduce(1).debug();

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
