use crate::solutions::{Result, Solution};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::multi::{count, many0, many1};
use nom::sequence::preceded;
use nom::{bytes::complete::tag, combinator::map_res, sequence::tuple, IResult};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    r#type: usize,
    data: PacketData,
}

#[derive(Debug, PartialEq)]
enum PacketData {
    Literal(usize),
    Op(Vec<Packet>),
}

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
            .fold(0, |result, group| result << 4 | group);

        Ok((input, value))
    }

    fn header_item(input: &str) -> IResult<&str, usize> {
        map_res(take(3usize), Self::from_bin)(input)
    }

    fn packet_literal(input: &str) -> IResult<&str, Packet> {
        let (input, (version, _, value)) =
            tuple((Self::header_item, tag("100"), Self::literal_groups))(input)?;

        Ok((
            input,
            Packet {
                version,
                r#type: 4,
                data: PacketData::Literal(value),
            },
        ))
    }

    fn packet_op1(input: &str) -> IResult<&str, Packet> {
        let (input, (version, r#type, len)) = tuple((
            Self::header_item,
            Self::header_item,
            preceded(tag("0"), map_res(take(15usize), Self::from_bin)),
        ))(input)?;
        let (input, s) = take(len)(input)?;
        let (_, packets) = many1(Self::packet)(s)?;

        Ok((
            input,
            Packet {
                version,
                r#type,
                data: PacketData::Op(packets),
            },
        ))
    }

    fn packet_op2(input: &str) -> IResult<&str, Packet> {
        let (input, (version, r#type, len)) = tuple((
            Self::header_item,
            Self::header_item,
            preceded(tag("1"), map_res(take(11usize), Self::from_bin)),
        ))(input)?;
        let (input, packets) = count(Self::packet, len as usize)(input)?;

        Ok((
            input,
            Packet {
                version,
                r#type,
                data: PacketData::Op(packets),
            },
        ))
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

impl Packet {
    fn sum(&self) -> usize {
        match &self.data {
            PacketData::Literal(_) => self.version,
            PacketData::Op(sub) => sub
                .iter()
                .fold(self.version, |sum, packet| sum + packet.sum()),
        }
    }

    fn value(&self) -> usize {
        match &self.data {
            PacketData::Literal(value) => *value,
            PacketData::Op(packets) => match self.r#type {
                0 => packets.iter().map(|p| p.value()).sum(),
                1 => packets.iter().map(|p| p.value()).product(),
                2 => packets.iter().map(|p| p.value()).min().unwrap(),
                3 => packets.iter().map(|p| p.value()).max().unwrap(),
                5 => (packets[0].value() > packets[1].value()) as usize,
                6 => (packets[0].value() < packets[1].value()) as usize,
                7 => (packets[0].value() == packets[1].value()) as usize,
                _ => unreachable!(),
            },
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
