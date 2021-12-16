use crate::solutions::{Result, Solution};
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug)]
enum Packet {
    Literal(usize, usize, usize),
    Op(usize, usize, Vec<Packet>),
}

trait BinString {
    fn as_bits(&self) -> Vec<u8>;
}

impl BinString for str {
    fn as_bits(&self) -> Vec<u8> {
        self.chars()
            .flat_map(|b| {
                let digit = b.to_digit(16).unwrap();
                (0..4).map(move |i| ((digit << (i + 1) & 0b10000) >> 4) as u8)
            })
            .collect::<Vec<_>>()
    }
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

trait Bits {
    fn value(&self, from: usize, n: usize) -> usize;
}

impl Bits for [u8] {
    fn value(&self, from: usize, n: usize) -> usize {
        self[from..from + n]
            .iter()
            .fold(0, |r, b| (r << 1) + *b as usize)
    }
}

impl DaySolution {
    fn read_packets(&self, bits: &[u8], count: usize) -> (usize, Vec<Packet>) {
        let mut index = 0;
        let mut packets = vec![];

        while index < bits.len() && packets.len() < count {
            let version = bits.value(index, 3);
            index += 3;

            let typ = bits.value(index, 3);
            index += 3;

            if typ == 4 {
                let mut literals: Vec<usize> = vec![];
                while bits[index] == 1 {
                    let literal = bits.value(index + 1, 4);
                    index += 5;

                    literals.push(literal);
                }
                let literal = bits.value(index + 1, 4);
                index += 5;

                literals.push(literal);

                let value = literals.iter().fold(0, |r, b| (r << 4) + *b);

                packets.push(Packet::Literal(version, typ, value as usize));
                continue;
            }

            let op_type = bits[index];
            index += 1;

            let index_inc = if op_type == 0 { 15 } else { 11 };
            let op_len = bits.value(index, index_inc);
            index += index_inc;

            let (bits_range, packets_to_read) = if op_type == 0 {
                (&bits[index..index + op_len], usize::MAX)
            } else {
                (&bits[index..], op_len)
            };

            let (skip, sub_packets) = self.read_packets(bits_range, packets_to_read);
            index += skip;
            packets.push(Packet::Op(version, typ, sub_packets))
        }

        (index, packets)
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (_, packets) = self.read_packets(&input.unwrap().as_bits(), 1);
        let sum = packets[0].sum();

        Ok(Box::new(sum))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let (_, packets) = self.read_packets(&input.unwrap().as_bits(), 1);

        Ok(Box::new(packets[0].value()))
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
