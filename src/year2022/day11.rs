use crate::harness::Harness;
use std::str::Lines;

pub struct Solution;

impl Harness for Solution {
    type Parsed = (Vec<Monkey>, Vec<Vec<i64>>);
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        let mut lines = raw_input.lines();
        let mut monkeys = Vec::new();
        let mut all_items = Vec::new();

        while let Some((monkey, items)) = Monkey::parse(&mut lines) {
            monkeys.push(monkey);
            all_items.push(items);
            lines.next();
        }

        (monkeys, all_items)
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let monkeys = &input.0;
        let mut all_items = input.1.clone();
        let mut inspections = vec![0; monkeys.len()];

        const ROUNDS: usize = 20;
        for _ in 0..ROUNDS {
            for (i, monkey) in monkeys.iter().enumerate() {
                let items: Vec<_> = all_items[i].drain(..).collect();
                inspections[i] += items.len();

                for item in items.into_iter() {
                    let new = monkey.operation.apply(item) / 3;

                    let target = if new % monkey.test_divisor == 0 {
                        monkey.true_target
                    } else {
                        monkey.false_target
                    };

                    all_items[target].push(new);
                }
            }
        }

        inspections.sort_unstable();
        inspections.into_iter().rev().take(2).product()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let monkeys = &input.0;
        let mut all_items = input.1.clone();
        let mut inspections = vec![0; monkeys.len()];

        let combined_divisor: i64 = monkeys.iter().map(|m| m.test_divisor).product();

        const ROUNDS: usize = 10_000;
        for _ in 0..ROUNDS {
            for (i, monkey) in monkeys.iter().enumerate() {
                let items: Vec<_> = all_items[i].drain(..).collect();
                inspections[i] += items.len();

                for item in items.into_iter() {
                    let new = monkey.operation.apply(item) % combined_divisor;

                    let target = if new % monkey.test_divisor == 0 {
                        monkey.true_target
                    } else {
                        monkey.false_target
                    };

                    all_items[target].push(new);
                }
            }
        }

        inspections.sort_unstable();
        inspections.into_iter().rev().take(2).product()
    }
}

pub struct Monkey {
    operation: Operation,
    test_divisor: i64,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn parse(lines: &mut Lines<'_>) -> Option<(Self, Vec<i64>)> {
        // Monkey #
        lines.next()?;

        // Starting items
        let items: Vec<i64> = lines
            .next()?
            .split_once(": ")?
            .1
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        // Operation
        let mut op = lines.next()?.split_once("= ")?.1.split(' ');
        op.next()?; // "old"
        let operation = match (op.next()?, op.next()?) {
            ("+", "old") => Operation::Relative(Operator::Add),
            ("+", n) => Operation::Constant(Operator::Add, n.parse().ok()?),
            ("*", "old") => Operation::Relative(Operator::Multiply),
            ("*", n) => Operation::Constant(Operator::Multiply, n.parse().ok()?),
            _ => unreachable!(),
        };

        // Test divisor
        let test_divisor: i64 = lines.next()?.split_once(" by ")?.1.parse().ok()?;

        // Targets
        let true_target: usize = lines.next()?.rsplit(' ').next()?.parse().ok()?;
        let false_target: usize = lines.next()?.rsplit(' ').next()?.parse().ok()?;

        Some((
            Monkey {
                operation,
                test_divisor,
                true_target,
                false_target,
            },
            items,
        ))
    }
}

pub enum Operation {
    Constant(Operator, i64),
    Relative(Operator),
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Self::Constant(Operator::Add, n) => old + n,
            Self::Constant(Operator::Multiply, n) => old * n,
            Self::Relative(Operator::Add) => old + old,
            Self::Relative(Operator::Multiply) => old * old,
        }
    }
}

pub enum Operator {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day11-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 10605);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day11-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 2713310158);
    }
}
