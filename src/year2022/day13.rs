use crate::harness::Harness;
use std::cmp::Ordering;
use std::str::FromStr;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<Packet>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        let mut lines = raw_input.lines();
        let mut packets = Vec::new();

        while let Some((s1, s2)) = lines.next().zip(lines.next()) {
            let p1: Packet = s1.parse().unwrap();
            let p2: Packet = s2.parse().unwrap();
            packets.push(p1);
            packets.push(p2);

            lines.next();
        }

        packets
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        input
            .chunks(2)
            .enumerate()
            .filter_map(|(i, p)| if p[0] < p[1] { Some(i + 1) } else { None })
            .sum()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let mut packets = input.clone();
        let divider1: Packet = "[[2]]".parse().unwrap();
        let divider2: Packet = "[[6]]".parse().unwrap();

        packets.push(divider1.clone());
        packets.push(divider2.clone());
        packets.sort_unstable();

        packets
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if p == &divider1 || p == &divider2 {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .product()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = ParseState::Start;
        for c in s.chars() {
            match (c, state) {
                ('[', prev_state) => {
                    state = ParseState::InList(Vec::new(), Box::new(prev_state));
                }
                (']', ParseState::InList(packets, prev_state)) => match *prev_state {
                    ParseState::Start => return Ok(Packet::List(packets)),
                    ParseState::InList(mut pk, prev_state) => {
                        pk.push(Packet::List(packets));
                        state = ParseState::InList(pk, prev_state);
                    }
                    _ => unreachable!(),
                },
                (']', ParseState::Number(number, prev_state)) => match *prev_state {
                    ParseState::InList(mut packets, prev_state) => {
                        packets.push(Packet::Int(number));
                        match *prev_state {
                            ParseState::Start => return Ok(Packet::List(packets)),
                            ParseState::InList(mut pk, prev_state) => {
                                pk.push(Packet::List(packets));
                                state = ParseState::InList(pk, prev_state);
                            }
                            ParseState::Number(_, _) => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                },
                (',', ParseState::Number(number, prev_state)) => match *prev_state {
                    ParseState::InList(mut packets, prev_state) => {
                        packets.push(Packet::Int(number));
                        state = ParseState::InList(packets, prev_state);
                    }
                    _ => unreachable!(),
                },
                (',', s) => {
                    state = s;
                }
                (n @ '0'..='9', ParseState::Number(number, prev_state)) => {
                    let digit = n.to_digit(10).unwrap();
                    state = ParseState::Number(number * 10 + digit, prev_state);
                }
                (n @ '0'..='9', prev_state) => {
                    let digit = n.to_digit(10).unwrap();
                    state = ParseState::Number(digit, Box::new(prev_state));
                }
                _ => unreachable!(),
            }
        }

        match state {
            ParseState::InList(l, _) => Ok(Packet::List(l)),
            _ => unreachable!(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                loop {
                    match (l_iter.next(), r_iter.next()) {
                        (Some(l), Some(r)) => match l.cmp(r) {
                            Ordering::Equal => continue,
                            o => return o,
                        },
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (l @ Packet::Int(_), r @ Packet::List(_)) => Packet::List(vec![l.clone()]).cmp(r),
            (l @ Packet::List(_), r @ Packet::Int(_)) => l.cmp(&Packet::List(vec![r.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
enum ParseState {
    Start,
    InList(Vec<Packet>, Box<ParseState>),
    Number(u32, Box<ParseState>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day13-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 13);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day13-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 140);
    }
}
