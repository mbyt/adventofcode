use std::{collections::HashMap, hash::Hash};

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Default for Pos {
    // avoid negative values, start with a real big one
    fn default() -> Self {
        Self {
            x: 1_000_000,
            y: 1_000_000,
        }
    }
}

pub fn main() {
    let example = r#"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "#
    .trim();
    let moves: Vec<Move> = example
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 2);
            let count: i32 = parts[1].parse().unwrap();
            match parts[0] {
                "R" => Move::Right(count),
                "L" => Move::Left(count),
                "D" => Move::Down(count),
                "U" => Move::Up(count),
                _ => {
                    panic!("Unknown parts[0]={}", parts[0]);
                }
            }
        })
        .collect();

    let mut head = Pos::default();
    let mut tail = Pos::default();
    let mut visited: HashMap<Pos, i32> = [(tail, 1)].into_iter().collect();
    for move_ in moves {
        assert!(head.x > 0);
        assert!(head.y > 0);
        match move_ {
            Move::Right(count) | Move::Left(count) => {
                let sign = if matches!(move_, Move::Up(_)) {1} else { -1 };
                for _i in 0..count {
                    head = Pos {
                        x: head.x + 1*sign,
                        ..head
                    };
                    // TODO: check left and right
                    // TODO: what if positions exaclty match?
                    if head.y == tail.y {
                        if head.x - tail.x == 2 {
                            tail.x += 1;
                        } else if tail.x - head.x == 2 {
                            tail.x -= 1;
                        } else {
                            // TODO: uncomment: unclear what happened here
                            // panic!("Unexpected position");
                        }
                    } else if head.x == tail.x {
                        if head.y - tail.y == 2 {
                            tail.y += 1;
                        } else if tail.y - head.y == 2 {
                            tail.y -= 1;
                        } else {
                            panic!("Unexpected position");
                        }
                    } else {
                        panic!("Not implemented yet");
                    }
                }
            },
            Move::Up(count) | Move::Down(count) => {
                let sign = if matches!(move_, Move::Up(_)) {1} else { -1 };
                for _i in 0..count {
                    head = Pos {
                        x: head.x + 1*sign,
                        ..head
                    };
                }
            },
        }
        visited
            .entry(tail)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    println!("Hello, world!");
}
