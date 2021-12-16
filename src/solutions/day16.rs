use crate::solutions::{Result, Solution};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::multi::{count, many0, many1};
use nom::sequence::preceded;
use nom::{bytes::complete::tag, combinator::map_res, sequence::tuple, IResult};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

trait BitString {
    fn to_bit_string(&self) -> String;
}

impl BitString for str {
    fn to_bit_string(&self) -> String {
        self.chars()
            .map(|ch| {
                let digit = ch.to_digit(16).unwrap();
                format!("{:04b}", digit & 0b1111)
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

struct Parser {}

impl Parser {
    fn from_bin(input: &str) -> std::result::Result<usize, std::num::ParseIntError> {
        usize::from_str_radix(input, 2)
    }

    fn literal_group(prefix: &str) -> impl Fn(&str) -> IResult<&str, usize> + '_ {
        move |input| preceded(tag(prefix), map_res(take(4usize), Self::from_bin))(input)
    }

    fn literal_groups(input: &str) -> IResult<&str, usize> {
        let (input, (mut head, tail)) = tuple((
            many0(Parser::literal_group("1")),
            Parser::literal_group("0"),
        ))(input)?;

        head.push(tail);
        let value = head
            .into_iter()
            .fold(0, |result, group| (result << 4) + group as usize);

        Ok((input, value))
    }

    fn header_item(input: &str) -> IResult<&str, usize> {
        map_res(take(3usize), Self::from_bin)(input)
    }

    fn packet_literal(input: &str) -> IResult<&str, Packet> {
        let (input, (version, _, value)) =
            tuple((Self::header_item, tag("100"), Self::literal_groups))(input)?;

        Ok((input, Packet::Literal(version, 4, value)))
    }

    fn packet_op1(input: &str) -> IResult<&str, Packet> {
        let (input, (version, r#type, _, len)) = tuple((
            Self::header_item,
            Self::header_item,
            tag("0"),
            map_res(take(15usize), Self::from_bin),
        ))(input)?;
        let (input, s) = take(len)(input)?;
        let (_, packets) = many1(Self::packet)(s)?;

        Ok((input, Packet::Op(version, r#type, packets)))
    }

    fn packet_op2(input: &str) -> IResult<&str, Packet> {
        let (input, (version, r#type, _, len)) = tuple((
            Self::header_item,
            Self::header_item,
            tag("1"),
            map_res(take(11usize), Self::from_bin),
        ))(input)?;
        let (input, packets) = count(Self::packet, len as usize)(input)?;

        Ok((input, Packet::Op(version, r#type, packets)))
    }

    fn packet(input: &str) -> IResult<&str, Packet> {
        alt((Parser::packet_literal, Self::packet_op1, Self::packet_op2))(input)
    }

    fn parse(input: &str) -> Option<Packet> {
        Self::packet(&input.to_bit_string())
            .map(|(_, packet)| packet)
            .ok()
    }
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal(usize, usize, usize),
    Op(usize, usize, Vec<Packet>),
}

impl Packet {
    fn sum(&self) -> usize {
        match self {
            Packet::Literal(version, _, _) => *version,
            Packet::Op(version, _, sub) => sub.iter().fold(*version, |a, c| a + c.sum()),
        }
    }

    fn value(&self) -> usize {
        match self {
            Packet::Literal(_, _, value) => *value,
            Packet::Op(_, 0, sub) => sub.iter().map(|p| p.value()).sum(),
            Packet::Op(_, 1, sub) => sub.iter().map(|p| p.value()).product(),
            Packet::Op(_, 2, sub) => sub.iter().map(|p| p.value()).min().unwrap(),
            Packet::Op(_, 3, sub) => sub.iter().map(|p| p.value()).max().unwrap(),
            Packet::Op(_, 5, sub) => (sub[0].value() > sub[1].value()) as usize,
            Packet::Op(_, 6, sub) => (sub[0].value() < sub[1].value()) as usize,
            Packet::Op(_, 7, sub) => (sub[0].value() == sub[1].value()) as usize,
            _ => unreachable!(),
        }
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let packet = Parser::parse(&input.unwrap()).unwrap();
        Ok(Box::new(packet.sum()))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let packet = Parser::parse(&input.unwrap()).unwrap();
        Ok(Box::new(packet.value()))
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::DaySolution;
    use crate::Solution;

    #[test]
    fn part_1() {
        let result = DaySolution::default()
            .part_1(Some("8A004A801A8002F478".to_string()))
            .unwrap();

        assert_eq!("16", result.to_string());

        let result = DaySolution::default()
            .part_1(Some(include_str!("../../inputs/day16.txt").to_string()))
            .unwrap();

        assert_eq!("951", result.to_string());
    }

    #[test]
    fn part_2() {
        let result = DaySolution::default()
            .part_2(Some("C200B40A82".to_string()))
            .unwrap();

        assert_eq!("3", result.to_string());

        let result = DaySolution::default()
            .part_2(Some(include_str!("../../inputs/day16.txt").to_string()))
            .unwrap();

        assert_eq!("902198718880", result.to_string());
    }
}
