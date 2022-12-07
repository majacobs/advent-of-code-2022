use crate::harness::Harness;
use std::str::Lines;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Directory;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        let mut root = Directory { contents: vec![] };
        let mut lines = raw_input.lines();
        lines.next(); // Skip "$ cd /"
        parse(&mut lines, &mut root);
        root
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut sum = 0;
        let mut to_visit = vec![input];
        while let Some(dir) = to_visit.pop() {
            let size = dir.recursive_size();
            if size <= 100_000 {
                sum += size;
            }
            for entry in dir.contents.iter() {
                if let Entry::Directory(d) = entry {
                    to_visit.push(d);
                }
            }
        }
        sum
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        const TOTAL_SPACE: u32 = 70_000_000;
        const SPACE_NEEDED: u32 = 30_000_000;

        let to_free = input.recursive_size() + SPACE_NEEDED - TOTAL_SPACE;
        let mut smallest = u32::MAX;

        let mut to_visit = vec![input];
        while let Some(dir) = to_visit.pop() {
            let size = dir.recursive_size();
            if size < smallest && size >= to_free {
                smallest = size;
            }
            for entry in dir.contents.iter() {
                if let Entry::Directory(d) = entry {
                    to_visit.push(d);
                }
            }
        }

        smallest
    }
}

fn parse(lines: &mut Lines<'_>, cwd: &mut Directory) {
    while let Some(line) = lines.next() {
        match line.split_once(' ').unwrap() {
            ("$", "ls") => {}
            ("$", command) => match command.split_once(' ').unwrap() {
                ("cd", "..") => return,
                ("cd", _name) => {
                    let mut subdir = Directory { contents: vec![] };
                    parse(lines, &mut subdir);
                    cwd.contents.push(Entry::Directory(subdir));
                }
                _ => unreachable!(),
            },
            ("dir", _name) => {}
            (size, _name) => {
                let size: u32 = size.parse().unwrap();
                cwd.contents.push(Entry::File(File { size }))
            }
        }
    }
}

#[derive(Debug)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
pub struct Directory {
    contents: Vec<Entry>,
}

impl Directory {
    fn recursive_size(&self) -> u32 {
        self.contents
            .iter()
            .map(|entry| match entry {
                Entry::Directory(d) => d.recursive_size(),
                Entry::File(f) => f.size,
            })
            .sum()
    }
}

#[derive(Debug)]
pub struct File {
    size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day07-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 95437);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day07-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 24933642);
    }
}
