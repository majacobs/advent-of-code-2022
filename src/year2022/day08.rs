use crate::harness::Harness;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub struct Solution;

impl Harness for Solution {
    type Parsed = (usize, Vec<Tree>);
    type Part1Output = usize;
    type Part2Output = u32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        let mut width = 0;
        let mut height = 0;
        let mut combined = Vec::new();
        for (i, line) in raw_input.lines().enumerate() {
            width = line.len();
            height = i + 1;
            for (j, c) in line.bytes().enumerate() {
                combined.push(Tree {
                    height: c - b'0',
                    visible: i == 0 || j == 0 || j == width - 1,
                });
            }
        }

        for tree in combined.iter_mut().skip(width * (height - 1)) {
            tree.visible = true;
        }

        (width, combined)
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let (width, mut trees) = input.clone();

        for row in trees.chunks_mut(width) {
            let mut tallest_left = 0;
            for tree in row.iter_mut() {
                if tree.height > tallest_left {
                    tree.visible = true;
                    tallest_left = tree.height;
                }
            }

            let mut tallest_right = 0;
            for tree in row.iter_mut().rev() {
                if tree.height > tallest_right {
                    tree.visible = true;
                    tallest_right = tree.height;
                }
            }
        }

        for i in 0..width {
            let mut tallest_top = 0;
            for tree in trees.iter_mut().skip(i).step_by(width) {
                if tree.height > tallest_top {
                    tree.visible = true;
                    tallest_top = tree.height;
                }
            }

            let mut tallest_bottom = 0;
            for tree in trees.iter_mut().skip(i).step_by(width).rev() {
                if tree.height > tallest_bottom {
                    tree.visible = true;
                    tallest_bottom = tree.height;
                }
            }
        }

        trees.iter().filter(|t| t.visible).count()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let width = input.0;
        let trees = &input.1;

        trees
            .par_iter()
            .enumerate()
            .map(|(index, tree)| {
                let row = index / width;
                let col = index % width;

                let mut a = 0;
                if col > 0 {
                    for i in (0..=(col - 1)).rev() {
                        a += 1;
                        let neighbor = trees[row * width + i];
                        if neighbor.height >= tree.height {
                            break;
                        }
                    }
                }

                let mut b = 0;
                for i in (col + 1)..width {
                    b += 1;
                    let neighbor = trees[row * width + i];
                    if neighbor.height >= tree.height {
                        break;
                    }
                }

                let mut c = 0;
                if row > 0 {
                    for i in (0..=(row - 1)).rev() {
                        c += 1;
                        let neighbor = trees[i * width + col];
                        if neighbor.height >= tree.height {
                            break;
                        }
                    }
                }

                let mut d = 0;
                for i in (row + 1)..width {
                    d += 1;
                    let neighbor = trees[i * width + col];
                    if neighbor.height >= tree.height {
                        break;
                    }
                }

                a * b * c * d
            })
            .max()
            .unwrap_or_default()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tree {
    height: u8,
    visible: bool,
}

unsafe impl Send for Tree {}
unsafe impl Sync for Tree {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day08-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 21);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day08-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 8);
    }
}
