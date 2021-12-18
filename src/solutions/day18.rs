use crate::solutions::{Result, Solution};
use nom::branch::alt;
use nom::bytes::streaming::take_while1;
use nom::character::complete::char;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{combinator::map_res, IResult};
use std::fmt::Display;
use crate::day18::SnailFish::Pair;

// #[derive(Debug)]
// enum SnailFishPart {
//     Value(i32),
//     Array(Vec<SnailFish2>),
// }
//
// type SnailFish2 = (SnailFishPart, SnailFishPart);

#[derive(Debug, Clone)]
enum SnailFish {

    Pair(Box<SnailFish>, Box<SnailFish>),
    Value(i32),
}

impl SnailFish {
    // fn add(&self) -> SnailFish {}

    fn reduce(&self, level: usize) -> SnailFish {
        // [[[[[9,8],1],2],3],4] -> [[[[0,9],2],3],4]
        match self {
            Self::Pair(a, b) if level < 4 => {
                Self::Pair(Box::new(a.reduce(level + 1)), Box::new(b.reduce(level + 1)))
            }
            Self::Pair(a, b) if level > 4 => {
                unimplemented!();
                if let Self::Value(a) =
            }
            _ => self,
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

        Ok((
            input,
            SnailFish::Pair(Box::new(SnailFish::Value(a)), Box::new(SnailFish::Value(b))),
        ))
    }

    fn pair2(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::value, char(','), Self::snailfish),
                char(']'),
            ),
        )(input)?;

        Ok((
            input,
            SnailFish::Pair(Box::new(SnailFish::Value(a)), Box::new(b)),
        ))
    }

    fn pair3(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::snailfish, char(','), Self::value),
                char(']'),
            ),
        )(input)?;

        Ok((
            input,
            SnailFish::Pair(Box::new(a), Box::new(SnailFish::Value(b))),
        ))
    }

    fn pair4(input: &str) -> IResult<&str, SnailFish> {
        let (input, (a, b)) = preceded(
            char('['),
            terminated(
                separated_pair(Self::snailfish, char(','), Self::snailfish),
                char(']'),
            ),
        )(input)?;

        Ok((input, SnailFish::Pair(Box::new(a), Box::new(b))))
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
        println!("{:?}", Parser::parse("[1,2]"));
        println!("{:?}", Parser::parse("[[1,2],3]"));
        println!("{:?}", Parser::parse("[9,[8,7]]"));
        println!("{:?}", Parser::parse("[[1,9],[8,5]]"));
        println!("{:?}", Parser::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"));
        println!(
            "{:?}",
            Parser::parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")
        );

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
