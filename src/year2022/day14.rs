use crate::harness::Harness;
use std::collections::HashSet;

pub struct Solution;

impl Harness for Solution {
    type Parsed = HashSet<(u32, u32)>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        let mut points = HashSet::new();
        for line in raw_input.lines() {
            let vertices: Vec<_> = line
                .split(" -> ")
                .map(|segment| {
                    let mut parts = segment.split(',');
                    let x: u32 = parts.next().unwrap().parse().unwrap();
                    let y: u32 = parts.next().unwrap().parse().unwrap();
                    (x, y)
                })
                .collect();

            for window in vertices.windows(2) {
                let x_min = window[0].0.min(window[1].0);
                let x_max = window[0].0.max(window[1].0);
                let y_min = window[0].1.min(window[1].1);
                let y_max = window[0].1.max(window[1].1);
                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        points.insert((x, y));
                    }
                }
            }
        }

        points
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut occupied: HashSet<(u32, u32)> = input.clone();
        let max_y = *occupied.iter().map(|(_, y)| y).max().unwrap();

        'outer: loop {
            const START: (u32, u32) = (500, 0);
            let (mut x, mut y) = START;

            loop {
                if !occupied.contains(&(x, y + 1)) {
                    y += 1;
                    if y > max_y {
                        break 'outer;
                    }
                } else if !occupied.contains(&(x - 1, y + 1)) {
                    x -= 1;
                    y += 1;
                } else if !occupied.contains(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                } else {
                    break;
                }
            }

            occupied.insert((x, y));
        }

        occupied.len() - input.len()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let mut occupied: HashSet<(u32, u32)> = input.clone();
        let max_y = *occupied.iter().map(|(_, y)| y).max().unwrap();
        let floor = max_y + 2;

        loop {
            const START: (u32, u32) = (500, 0);
            let (mut x, mut y) = START;

            loop {
                if !occupied.contains(&(x, y + 1)) && y + 1 < floor {
                    y += 1;
                } else if !occupied.contains(&(x - 1, y + 1)) && y + 1 < floor {
                    x -= 1;
                    y += 1;
                } else if !occupied.contains(&(x + 1, y + 1)) && y + 1 < floor {
                    x += 1;
                    y += 1;
                } else {
                    break;
                }
            }

            occupied.insert((x, y));
            if (x, y) == START {
                break;
            }
        }

        occupied.len() - input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day14-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 24);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day14-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 93);
    }
}
