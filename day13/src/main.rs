use std::cmp::Ordering;
use PacketNode::{Integer, List};

#[derive(Clone)]
enum PacketNode {
    Integer(u32),
    List(Vec<PacketNode>),
}

impl From<&str> for PacketNode {
    fn from(value: &str) -> Self {
        let mut tokens = value.chars().skip(1);
        Self::parse_list(&mut tokens)
    }
}

impl Eq for PacketNode {}

impl Ord for PacketNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq<Self> for PacketNode {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl PartialOrd<Self> for PacketNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Integer(l), Integer(r)) => l.partial_cmp(r),
            (List(l), Integer(r)) => l.partial_cmp(&vec![Integer(*r); 1]),
            (Integer(l), List(r)) => vec![Integer(*l); 1].partial_cmp(r),
            (List(l), List(r)) => {
                for i in 0..l.len() {
                    if i == r.len() {
                        return Some(Ordering::Greater);
                    }

                    let result = l[i].partial_cmp(&r[i]);

                    if result != Some(Ordering::Equal) {
                        return result;
                    }
                }

                if l.len() == r.len() {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            }
        }
    }
}

impl PacketNode {
    fn parse_list(tokens: &mut impl Iterator<Item = char>) -> PacketNode {
        let mut list = Vec::new();
        let mut last_token = '_';

        loop {
            let token = tokens.next().unwrap();

            let node = match token {
                ']' => break,
                ',' => {
                    last_token = token;
                    continue;
                }
                '[' => Self::parse_list(tokens),
                i => {
                    if last_token.is_ascii_digit() {
                        list.pop();
                        Integer(last_token.to_digit(10).unwrap() * 10 + i.to_digit(10).unwrap())
                    } else {
                        Integer(i.to_digit(10).unwrap())
                    }
                }
            };

            list.push(node);
            last_token = token;
        }

        List(list)
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut packets = INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(PacketNode::from)
        .collect::<Vec<_>>();

    let total = packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .fold(0, |a, (i, _)| a + i + 1);

    println!("Part 1: {total}");

    let divider_1 = PacketNode::from("[[2]]");
    let divider_2 = PacketNode::from("[[6]]");

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();

    let divider_1_index = packets.iter().position(|p| p == &divider_1).unwrap() + 1;
    let divider_2_index = packets.iter().position(|p| p == &divider_2).unwrap() + 1;

    println!("Part 2: {}", divider_1_index * divider_2_index);
}
