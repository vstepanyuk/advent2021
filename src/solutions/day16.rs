use crate::solutions::{Result, Solution};
use clap::Values;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Default)]
pub struct DaySolution;

#[derive(Debug)]
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
        if let Packet::Literal(_, _, value) = self {
            print!("{} ", value);
        }
        // if let Packet::Op(_, n, _) = self {
        //     print!("{}", n);
        // }

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
    fn bits_take(&self, from: usize, n: usize) -> usize;
    fn print_bits(&self);
}

impl Bits for Vec<u8> {
    fn bits_take(&self, from: usize, n: usize) -> usize {
        self[from..from + n]
            .iter()
            .fold(0, |r, b| (r << 1) + *b as usize)
    }

    fn print_bits(&self) {
        self.iter().for_each(|x| print!("{}", x));
        println!();
    }
}

impl Bits for &[u8] {
    fn bits_take(&self, from: usize, n: usize) -> usize {
        self[from..from + n]
            .iter()
            .fold(0, |r, b| (r << 1) + *b as usize)
    }

    fn print_bits(&self) {
        self.iter().for_each(|x| print!("{}", x));
        println!();
    }
}

impl DaySolution {
    fn read_packets(&self, bits: &[u8], count: usize) -> (usize, Vec<Packet>) {
        let mut index = 0;
        let mut packets = vec![];

        while index < bits.len() && packets.len() < count {
            println!();
            dbg!(index);
            let version = bits.bits_take(index, 3);
            index += 3;

            let typ = bits.bits_take(index, 3);
            index += 3;

            dbg!(version, typ);

            if typ == 4 {
                let mut literals: Vec<usize> = vec![];
                while bits[index] == 1 {
                    let literal = bits.bits_take(index + 1, 4);
                    index += 5;

                    literals.push(literal);
                }
                let literal = bits.bits_take(index + 1, 4);
                index += 5;

                literals.push(literal);

                let value = literals.iter().fold(0, |r, b| (r << 4) + *b);
                dbg!(value);

                packets.push(Packet::Literal(version, typ, value as usize));
                continue;
            }

            let op_type = bits[index];
            index += 1;

            let op_len = if op_type == 0 {
                bits.bits_take(index, 15) as usize
            } else {
                bits.bits_take(index, 11) as usize
            };
            dbg!(op_type, op_len);
            //
            index += if op_type == 0 { 15 } else { 11 };

            if op_type == 0 {
                // bits
                let bits = bits
                    .to_vec()
                    .iter()
                    .skip(index)
                    .take(op_len)
                    .map(|v| *v)
                    .collect::<Vec<u8>>();

                let result = self.read_packets(&bits, usize::MAX);

                index += result.0;
                packets.push(Packet::Op(version, typ, result.1))
            } else {
                let bits = bits
                    .to_vec()
                    .iter()
                    .skip(index)
                    .map(|v| *v)
                    .collect::<Vec<u8>>();

                let result = self.read_packets(&bits, op_len);

                index += result.0;
                packets.push(Packet::Op(version, typ, result.1))
            }
        }

        (index, packets)
    }
}

impl Solution for DaySolution {
    fn part_1(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let encoded = input.unwrap();

        let mut bits = encoded
            .chars()
            .flat_map(|b| {
                let digit = b.to_digit(16).unwrap();
                let mut digit = (digit & 0b1111) as u8;

                let mut bits = vec![];
                for _ in 0..4 {
                    bits.push(digit & 1);

                    digit = digit >> 1;
                }

                bits.reverse();
                bits
            })
            .collect::<Vec<_>>();

        let (_, packets) = self.read_packets(&bits, 1);
        let sum = packets[0].sum();

        Ok(Box::new(sum))
    }

    fn part_2(&mut self, input: Option<String>) -> Result<Box<dyn Display>> {
        let encoded = input.unwrap();

        let mut bits = encoded
            .chars()
            .flat_map(|b| {
                let digit = b.to_digit(16).unwrap();
                let mut digit = (digit & 0b1111) as u8;

                let mut bits = vec![];
                for _ in 0..4 {
                    bits.push(digit & 1);

                    digit = digit >> 1;
                }

                bits.reverse();
                bits
            })
            .collect::<Vec<_>>();

        let (_, packets) = self.read_packets(&bits, 1);

        Ok(Box::new(packets[0].value()))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::day16::DaySolution;
//     use crate::Solution;
//
//     #[test]
//     fn part_1() {
//         let input = include_str!("../../inputs/day16_demo.txt");
//         let result = DaySolution::default()
//             .part_1(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
//
//     #[test]
//     fn part_2() {
//         let input = include_str!("../../inputs/day16_demo.txt");
//         let result = DaySolution::default()
//             .part_2(Some(input.to_string()))
//             .unwrap();
//
//         assert_eq!("", result.to_string())
//     }
// }
