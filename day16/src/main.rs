use std::collections::VecDeque;

struct BitBuffer<'a> {
    buffer: u32,
    num_bits: u32,
    data: &'a mut VecDeque<u8>,
}

impl<'a> BitBuffer<'a> {
    fn new(data: &mut VecDeque<u8>) -> BitBuffer {
        BitBuffer {
            buffer: 0,
            num_bits: 0,
            data,
        }
    }

    fn fill(&mut self) {
        while self.num_bits < 28 {
            if self.data.len() == 0 {
                break;
            }
            self.buffer |= (self.data.pop_front().unwrap() as u32) << (28 - self.num_bits);
            self.num_bits += 4;
        }
    }

    fn get_n_bits(&mut self, n: u32) -> u16 {
        if self.num_bits < 28 {
            self.fill();
        }
        let result = self.buffer >> (32 - n);
        self.buffer <<= n;
        self.num_bits -= n;

        return result as u16;
    }
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: u32,
        value: u64,
    },
    Operator {
        version: u32,
        type_id: u32,
        packets: Vec<Packet>,
    },
}

fn parse_literal(buffer: &mut BitBuffer) -> (u64, u32) {
    let mut bits_used = 3 + 3 + 5;

    let mut result = Vec::new();
    let mut val = buffer.get_n_bits(5);
    loop {
        result.push(val & ((1 << 4) - 1));
        if val >> 4 == 0 {
            break;
        }
        bits_used += 5;
        val = buffer.get_n_bits(5);
    }

    let mut final_val: u64 = 0;

    for i in 0..result.len() {
        final_val |= ((result[i] as u64) << ((result.len() - 1 - i) * 4)) as u64;
    }

    (final_val, bits_used)
}

fn parse_operator(buffer: &mut BitBuffer) -> (Vec<Packet>, u32) {
    let mut bits_used = 3 + 3 + 1;
    let length = buffer.get_n_bits(1);
    let mut result = Vec::new();
    match length {
        0 => {
            let num_bits = buffer.get_n_bits(15) as u32;
            bits_used += 15 + num_bits;

            let mut parsed_bits = 0;
            while parsed_bits < num_bits {
                let (packet, bits_for_packet) = parse_packet(buffer);
                result.push(packet);
                parsed_bits += bits_for_packet;
            }
        }
        1 => {
            let num_packets = buffer.get_n_bits(11) as u32;
            bits_used += 11;
            for _ in 0..num_packets {
                let (packet, bits_for_packet) = parse_packet(buffer);
                result.push(packet);
                bits_used += bits_for_packet;
            }
        }
        _ => panic!(),
    };

    (result, bits_used)
}

fn parse_packet(buffer: &mut BitBuffer) -> (Packet, u32) {
    let version = buffer.get_n_bits(3) as u32;
    let type_id = buffer.get_n_bits(3) as u32;
    let mut bits_used = 0;

    match type_id {
        4 => {
            let (val, bits) = parse_literal(buffer);
            bits_used += bits;
            return (
                Packet::Literal {
                    version,
                    value: val,
                },
                bits_used,
            );
        }
        _ => {
            let (val, bits) = parse_operator(buffer);
            bits_used += bits;

            return (
                Packet::Operator {
                    version,
                    type_id,
                    packets: val,
                },
                bits_used,
            );
        }
    }
}

fn get_version_sum(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, value: _ } => *version as u64,
        Packet::Operator {
            version,
            type_id: _,
            packets,
        } => {
            return *version as u64
                + packets
                    .iter()
                    .map(|packet| get_version_sum(packet))
                    .sum::<u64>()
        }
    }
}

fn calculate_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version: _, value } => *value,
        Packet::Operator {
            version: _,
            type_id,
            packets,
        } => match type_id {
            0 => packets
                .iter()
                .map(|packet| calculate_packet(packet))
                .sum::<u64>(),
            1 => packets
                .iter()
                .map(|packet| calculate_packet(packet))
                .product::<u64>(),
            2 => packets
                .iter()
                .map(|packet| calculate_packet(packet))
                .min()
                .unwrap(),
            3 => packets
                .iter()
                .map(|packet| calculate_packet(packet))
                .max()
                .unwrap(),
            _ => {
                let first = calculate_packet(&packets[0]);
                let second = calculate_packet(&packets[1]);

                match type_id {
                    5 => {
                        if first > second {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if first < second {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!(),
                }
            }
        },
    }
}

fn main() {
    let input = include_str!("day16.txt");
    let mut data = VecDeque::from(
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect::<Vec<u8>>(),
    );
    let mut buffer = BitBuffer::new(&mut data);

    let (packet, bits_used) = parse_packet(&mut buffer);

    println!("{:?}, {}", packet, bits_used);
    println!("{:?}", get_version_sum(&packet));
    println!("{:?}", calculate_packet(&packet));
}
