use std::{collections::VecDeque, fs, vec};

pub fn run() {
    let bits = fs::read_to_string("inputs/day16.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .map(hex_to_bin)
        .flatten()
        .collect::<Vec<_>>();

    let packet = Packet::read_packet_bits(&bits);
    // dbg!(&packet);
    let p1 = packet.version_sum;
    let p2 = packet.value;
    // println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    assert_eq!(p1, 960);
    assert_eq!(p2, 12301926782560);
}

//hexadecimal number to binary string
fn hex_to_bin(hex: u8) -> Vec<u8> {
    let mut bin = vec![];
    for i in 0..4 {
        bin.push((hex >> (3 - i)) & 1);
    }
    bin
}

#[derive(Debug)]
struct Packet {
    version_sum: u32,
    length: usize,
    value: u64,
}

impl Packet {
    fn read_packet_bits(bits: &[u8]) -> Packet {
        let mut packets = Vec::new();
        let version = bits[0..=2].iter().fold(0, |acc, &b| acc * 2 + b as u32);
        let type_id = bits[3..=5].iter().fold(0, |acc, &b| acc * 2 + b as u8);
        let mut length = 6;
        let value;
        let mut packet_bits: &[u8] = &bits[6..];

        if type_id == 4 {
            let mut literal: VecDeque<u8> = VecDeque::new();
            loop {
                let bit_group: &[u8] = &packet_bits[0..=4];
                length += 5;
                literal.extend(bit_group[1..=4].iter());
                packet_bits = &packet_bits[5..];
                if bit_group[0] == 0 {
                    break;
                }
            }
            //trim trailing zeros from vec
            while literal.front() == Some(&0) {
                literal.pop_front();
            }

            value = literal.iter().fold(0, |acc, &b| acc * 2 + b as u64);
        } else {
            let length_type = packet_bits[0];
            if length_type == 0 {
                let packet_length = packet_bits[1..=15]
                    .iter()
                    .fold(0, |acc, &b| acc * 2 + b as usize);
                length += packet_length + 16;
                packet_bits = &packet_bits[16..];
                let mut total_packet_length = 0;
                while packet_length != total_packet_length {
                    let packet = Packet::read_packet_bits(packet_bits);
                    total_packet_length += packet.length;
                    packet_bits = &packet_bits[packet.length..];
                    packets.push(packet);
                }
            } else {
                let sub_packet_count = packet_bits[1..=11]
                    .iter()
                    .fold(0, |acc, &b| acc * 2 + b as usize);
                packet_bits = &packet_bits[12..];
                for _ in 0..sub_packet_count {
                    let packet = Packet::read_packet_bits(packet_bits);
                    packet_bits = &packet_bits[packet.length..];
                    packets.push(packet);
                }
                length += packets.iter().map(|p| p.length).sum::<usize>() + 12;
            }
            value = Self::calculate_value(&packets, type_id);
        }

        let version_sum = version + packets.iter().map(|p| p.version_sum).sum::<u32>();

        Packet {
            version_sum,
            length,
            value,
        }
    }

    fn calculate_value(packets: &[Packet], operator: u8) -> u64 {
        match operator {
            0 => packets.iter().map(|p| p.value).sum::<u64>(),
            1 => packets.iter().map(|p| p.value).product::<u64>(),
            2 => packets.iter().map(|p| p.value).min().unwrap(),
            3 => packets.iter().map(|p| p.value).max().unwrap(),
            5 => {
                if packets[0].value > packets[1].value {
                    1
                } else {
                    0
                }
            }
            6 => {
                if packets[0].value < packets[1].value {
                    1
                } else {
                    0
                }
            }
            7 => {
                if packets[0].value == packets[1].value {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid operator"),
        }
    }
}
