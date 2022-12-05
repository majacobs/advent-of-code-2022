use std::fs;

pub fn run() {
    let (stacks, moves) = read();
    println!("Day 5");
    println!("  Part 1: {}", part1(stacks.clone(), &moves));
    println!("  Part 2: {}", part2(stacks, &moves));
}

fn read() -> (Vec<Vec<char>>, Vec<Move>) {
    let raw = fs::read_to_string("input/day05").expect("Unable to read input");
    let mut lines = raw.lines();

    let mut stacks = Vec::new();
    for line in &mut lines {
        if !line.contains('[') {
            break;
        }

        let crates = line.chars().skip(1).step_by(4);
        for (i, c) in crates.enumerate() {
            if c == ' ' {
                continue;
            }
            while i >= stacks.len() {
                stacks.push(Vec::new());
            }
            stacks[i].push(c);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut moves = Vec::new();

    lines.next(); // Skip blank line
    for line in &mut lines {
        let mut parts = line.split(' ').skip(1).step_by(2);
        moves.push(Move {
            qty: parts.next().unwrap().parse().unwrap(),
            src: parts.next().unwrap().parse().unwrap(),
            dst: parts.next().unwrap().parse().unwrap(),
        });
    }

    (stacks, moves)
}

fn part1(mut stacks: Vec<Vec<char>>, moves: &[Move]) -> String {
    for mv in moves {
        for _ in 0..mv.qty {
            let c = stacks[mv.src - 1].pop().unwrap();
            stacks[mv.dst - 1].push(c);
        }
    }

    let mut message = String::new();
    for mut s in stacks.into_iter() {
        message.push(s.pop().unwrap_or(' '))
    }

    message
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &[Move]) -> String {
    for mv in moves {
        let mut buf = Vec::new();
        for _ in 0..mv.qty {
            buf.push(stacks[mv.src - 1].pop().unwrap());
        }
        while let Some(c) = buf.pop() {
            stacks[mv.dst - 1].push(c);
        }
    }

    let mut message = String::new();
    for mut s in stacks.into_iter() {
        message.push(s.pop().unwrap_or(' '))
    }

    message
}

#[derive(Debug)]
struct Move {
    qty: usize,
    src: usize,
    dst: usize,
}
