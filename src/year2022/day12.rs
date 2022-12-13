use crate::common::dijkstra::{shortest_path, shortest_path_matching, Edge};
use crate::harness::Harness;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Input;
    type Part1Output = usize;
    type Part2Output = usize;

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
        let edges = make_adjacency_list(&input.map, |curr, next| next <= curr + 1);
        let start = input.start.row * input.map[0].len() + input.start.col;
        let end = input.end.row * input.map[0].len() + input.end.col;
        shortest_path(&edges, start, end).unwrap()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let edges = make_adjacency_list(&input.map, |curr, next| next >= curr - 1);
        let width = input.map[0].len();
        let start = input.end.row * width + input.end.col;
        let is_goal = |index: usize| {
            let r = index / width;
            let c = index % width;
            input.map[r][c] == b'a'
        };
        shortest_path_matching(&edges, start, is_goal).unwrap()
    }
}

fn make_adjacency_list<F>(map: &Vec<Vec<u8>>, can_transit: F) -> Vec<Vec<Edge>>
where
    F: Fn(u8, u8) -> bool,
{
    let height = map.len();
    let width = map[0].len();

    let mut all_edges = Vec::new();
    let make_index = |row: usize, col: usize| row * width + col;

    for row in 0..height {
        for col in 0..width {
            let value = map[row][col];
            let mut edges = Vec::new();

            let mut add_neighbor = |nr: usize, nc: usize| {
                let index = make_index(nr, nc);
                let next_value = map[nr][nc];
                if can_transit(value, next_value) {
                    edges.push(Edge::new(index, 1));
                }
            };

            if row > 0 {
                add_neighbor(row - 1, col);
            }

            if col > 0 {
                add_neighbor(row, col - 1);
            }

            if row < height - 1 {
                add_neighbor(row + 1, col);
            }

            if col < width - 1 {
                add_neighbor(row, col + 1);
            }

            all_edges.push(edges);
        }
    }

    all_edges
}

pub struct Input {
    start: Position,
    end: Position,
    map: Vec<Vec<u8>>,
}

struct Position {
    row: usize,
    col: usize,
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
