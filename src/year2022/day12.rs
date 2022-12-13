use crate::harness::Harness;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Input;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        let mut start = Position { row: 0, col: 0 };
        let mut end = Position { row: 0, col: 0 };
        let map = raw_input
            .lines()
            .enumerate()
            .map(|(r, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(c, b)| match b {
                        b'S' => {
                            start = Position { row: r, col: c };
                            b'a'
                        }
                        b'E' => {
                            end = Position { row: r, col: c };
                            b'z'
                        }
                        c => c,
                    })
                    .collect()
            })
            .collect();

        Input { start, end, map }
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        dijkstra(&input.map, input.start, input.end).unwrap()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        dijkstra_backwards(&input.map, input.end, b'a').unwrap()
    }
}

pub struct Input {
    start: Position,
    end: Position,
    map: Vec<Vec<u8>>,
}

fn dijkstra(map: &Vec<Vec<u8>>, start: Position, end: Position) -> Option<u32> {
    let height = map.len();
    let width = map[0].len();

    let mut distances: Vec<Vec<u32>> = (0..height)
        .map(|_| (0..width).map(|_| u32::MAX).collect())
        .collect();
    distances[start.row][start.col] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: start,
        distance: 0,
    });

    while let Some(State { pos, distance }) = heap.pop() {
        if pos == end {
            return Some(distance);
        }

        if distance > distances[pos.row][pos.col] {
            continue;
        }

        const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in DELTAS {
            let next_row = (pos.row as isize) + dr;
            let next_col = (pos.col as isize) + dc;

            if next_row < 0
                || next_row >= height as isize
                || next_col < 0
                || next_col >= width as isize
            {
                continue;
            }

            let curr_cost = map[pos.row][pos.col];
            let next_cost = map[next_row as usize][next_col as usize];
            if next_cost > curr_cost + 1 {
                continue;
            }

            let next = State {
                pos: Position {
                    row: next_row as usize,
                    col: next_col as usize,
                },
                distance: distance + 1,
            };
            if next.distance < distances[next.pos.row][next.pos.col] {
                distances[next.pos.row][next.pos.col] = next.distance;
                heap.push(next);
            }
        }
    }

    None
}

fn dijkstra_backwards(map: &Vec<Vec<u8>>, start: Position, end: u8) -> Option<u32> {
    let height = map.len();
    let width = map[0].len();

    let mut distances: Vec<Vec<u32>> = (0..height)
        .map(|_| (0..width).map(|_| u32::MAX).collect())
        .collect();
    distances[start.row][start.col] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: start,
        distance: 0,
    });

    while let Some(State { pos, distance }) = heap.pop() {
        if map[pos.row][pos.col] == end {
            return Some(distance);
        }

        if distance > distances[pos.row][pos.col] {
            continue;
        }

        const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in DELTAS {
            let next_row = (pos.row as isize) + dr;
            let next_col = (pos.col as isize) + dc;

            if next_row < 0
                || next_row >= height as isize
                || next_col < 0
                || next_col >= width as isize
            {
                continue;
            }

            let curr_cost = map[pos.row][pos.col];
            let next_cost = map[next_row as usize][next_col as usize];
            if next_cost < curr_cost - 1 {
                continue;
            }

            let next = State {
                pos: Position {
                    row: next_row as usize,
                    col: next_col as usize,
                },
                distance: distance + 1,
            };
            if next.distance < distances[next.pos.row][next.pos.col] {
                distances[next.pos.row][next.pos.col] = next.distance;
                heap.push(next);
            }
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: Position,
    distance: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.row, self.col).cmp(&(other.row, other.col))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day12-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 31);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day12-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 29);
    }
}
